use crate::{
    configuration::StripeSettings,
    data::Database,
    domain::{
        error::GlobalError,
        guest_order_items::GuestOrderItemList,
        guest_orders::GuestOrder,
        order_items::{OrderItem, OrderItemList},
        products::{Product, ProductId, ProductIdList, ProductList},
        register::Register,
        users::User,
    },
};
use secrecy::ExposeSecret;
use serde::Serialize;
use std::sync::Arc;
use stripe::{
    Account, AccountLink, AccountLinkType, CheckoutSession,
    CheckoutSessionBillingAddressCollection, CheckoutSessionId, CheckoutSessionMode,
    CheckoutSessionUiMode, Client, CreateAccountLink, CreateCheckoutSession,
    CreateCheckoutSessionLineItems, CreateCheckoutSessionShippingAddressCollection,
    CreateCheckoutSessionShippingAddressCollectionAllowedCountries, CreateCustomer,
    CreatePaymentLink, CreatePaymentLinkLineItems, CreatePrice, CreateProduct, Currency, Customer,
    CustomerId, Expandable, IdOrCreate, PaymentLink, Price, PriceId, Product as StripeProduct,
    ProductId as TheStripeProductId, StripeError,
};
use thiserror::Error;

pub type StripeSessionId = CheckoutSessionId;
pub type StripeCurrency = Currency;
pub type StripeIdCustomer = CustomerId;
pub type StripeIdProduct = TheStripeProductId;
pub type StripeIdPrice = PriceId;

#[derive(Error, Debug)]
pub enum StripeErr {
    #[error("Conversion Error: {0}")]
    ParseIdError(#[from] stripe::ParseIdError),
    #[error("Stripe error: {0}")]
    StripeError(#[from] StripeError),
    #[error("Other error: {0}")]
    Other(String),
}

enum MetaEntity<'a> {
    Product(&'a Product),
    User(&'a User),
    Register(&'a Register),
    Guest(&'a GuestOrder),
}

#[derive(Debug, Serialize, Clone)]
pub struct StripePublishableKey(String);

#[derive(Clone)]
pub struct StripeClient {
    pub client: Client,
    pub settings: StripeSettings,
    pub stripe_publishable_key: StripePublishableKey,
    pub meta: std::collections::HashMap<String, String>,
}

impl StripeClient {
    pub fn new(settings: &StripeSettings) -> Self {
        let client = stripe::Client::new(settings.stripe_secret_key.expose_secret());
        let meta_data =
            std::collections::HashMap::from([(String::from("async-stripe"), String::from("true"))]);
        let stripe_publishable_key: StripePublishableKey = StripePublishableKey(String::from(
            settings.stripe_publishable_key.expose_secret(),
        ));

        Self {
            client,
            settings: settings.clone(),
            stripe_publishable_key,
            meta: meta_data,
        }
    }

    // Create new Account for Stripe Connect
    async fn create_account_stripe_connect(&self) -> Result<Account, StripeErr> {
        let account = Account::create(
            &self.client,
            stripe::CreateAccount {
                type_: Some(stripe::AccountType::Express),
                capabilities: Some(stripe::CreateAccountCapabilities {
                    card_payments: Some(stripe::CreateAccountCapabilitiesCardPayments {
                        requested: Some(true),
                    }),
                    transfers: Some(stripe::CreateAccountCapabilitiesTransfers {
                        requested: Some(true),
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            },
        )
        .await?;

        Ok(account)
    }

    fn build_metadata(&self, entity: MetaEntity) -> std::collections::HashMap<String, String> {
        let mut meta = self.meta.clone();

        match entity {
            MetaEntity::Product(product) => {
                meta.insert("product_id".to_string(), product.id.as_ref().to_string());
                meta.insert(
                    "category_id".to_string(),
                    product.category_id.as_ref().to_string(),
                );
            }
            MetaEntity::User(user) => {
                meta.insert("user_id".to_string(), user.id.as_ref().to_string());
                meta.insert("email".to_string(), user.email.as_ref().to_string());
            }
            MetaEntity::Register(user) => {
                meta.insert("user_id".to_string(), user.id.as_ref().to_string());
                meta.insert("email".to_string(), user.email.as_ref().to_string());
            }
            MetaEntity::Guest(guest) => {
                meta.insert("guest_id".to_string(), guest.id.as_ref().to_string());
            }
        }

        meta
    }

    pub async fn create_guest_customer(
        &self,
        guest_order: &GuestOrder,
    ) -> Result<Customer, StripeErr> {
        let customer = Customer::create(
            &self.client,
            CreateCustomer {
                name: None,
                email: None,
                description: None,
                metadata: Some(self.build_metadata(MetaEntity::Guest(guest_order))),

                ..Default::default()
            },
        )
        .await?;

        Ok(customer)
    }

    pub async fn create_customer(&self, register: &Register) -> Result<Customer, StripeErr> {
        let customer = Customer::create(
            &self.client,
            CreateCustomer {
                name: Some(&register.get_full_name()),
                email: Some(register.email.as_ref()),
                description: None,
                metadata: Some(self.build_metadata(MetaEntity::Register(register))),

                ..Default::default()
            },
        )
        .await?;

        Ok(customer)
    }

    pub async fn get_customer(&self, user: &User) -> Result<Customer, StripeErr> {
        let customer = Customer::create(
            &self.client,
            CreateCustomer {
                name: Some(user.get_full_name().as_ref()),
                email: Some(user.email.as_ref()),
                description: None,
                metadata: Some(self.build_metadata(MetaEntity::User(user))),

                ..Default::default()
            },
        )
        .await?;

        Ok(customer)
    }

    async fn get_product(&self, product: &Product) -> Result<StripeProduct, StripeErr> {
        let stripe_product = {
            let mut create_product = CreateProduct::new(product.name.as_ref());
            create_product.metadata = Some(self.build_metadata(MetaEntity::Product(product)));

            StripeProduct::create(&self.client, create_product).await?
        };
        Ok(stripe_product)
    }

    pub async fn get_price(
        &self,
        product: &Product,
        stripe_currency: &StripeCurrency,
    ) -> Result<Price, StripeErr> {
        let stripe_product = self.get_product(product).await?;
        let stripe_price = {
            let mut create_price = CreatePrice::new(*stripe_currency);
            create_price.product = Some(IdOrCreate::Id(&stripe_product.id));
            create_price.metadata = None;
            create_price.unit_amount = Some(*product.price.as_ref());
            create_price.expand = &["product"];

            Price::create(&self.client, create_price).await?
        };
        Ok(stripe_price)
    }

    pub async fn display_checkout_session_info(&self, checkout_session: &CheckoutSession) {
        let line_items = checkout_session.line_items.clone().unwrap();

        println!(
            "created a {} checkout session for {} {:?} for {} {} at {}",
            checkout_session.payment_status,
            line_items.data[0].quantity.unwrap(),
            match line_items.data[0]
                .price
                .as_ref()
                .unwrap()
                .product
                .as_ref()
                .unwrap()
            {
                Expandable::Object(p) => p.name.as_ref().unwrap(),
                _ => panic!("product not found"),
            },
            checkout_session.amount_subtotal.unwrap() / 100,
            line_items.data[0].price.as_ref().unwrap().currency.unwrap(),
            checkout_session.url.clone().unwrap()
        );
    }

    async fn create_line_items(
        &self,
        order_items: &OrderItemList,
        db: &Arc<Database>,
        stripe_currency: &StripeCurrency,
    ) -> Result<Vec<CreateCheckoutSessionLineItems>, GlobalError> {
        // Fetch all product IDs for the order items in a single query
        let product_ids: Vec<ProductId> = order_items
            .as_ref()
            .iter()
            .map(|item| item.product_id)
            .collect();

        // Step 2: Fetch products by their IDs from the database
        let products: ProductList = ProductIdList::new(product_ids)
            .get_products_by_ids(db)
            .await?;

        let product_map: std::collections::HashMap<ProductId, Product> = products
            .into_inner()
            .into_iter()
            .map(|product| (product.id, product))
            .collect();

        let mut line_items = Vec::with_capacity(order_items.as_ref().len());

        for item in order_items.as_ref().iter() {
            // Look up the product for the current order item
            let product = product_map
                .get(&item.product_id)
                .ok_or_else(|| GlobalError::GlobalStrErr("Product Not Found".to_string()))?;

            // Get or create the Stripe price ID for the product
            let price_id = if let Some(price_id) = product.stripe_price_id.as_ref() {
                price_id.clone()
            } else {
                self.get_price(&product, stripe_currency).await?.id
            };

            // Create the line item
            line_items.push(CreateCheckoutSessionLineItems {
                quantity: Some(*item.quantity.as_ref() as u64),
                price: Some(price_id.to_string()),
                ..Default::default()
            });
        }

        Ok(line_items)
    }

    async fn create_guest_line_items(
        &self,
        order_items: &GuestOrderItemList,
        db: &Arc<Database>,
        stripe_currency: &StripeCurrency,
    ) -> Result<Vec<CreateCheckoutSessionLineItems>, GlobalError> {
        // Fetch all product IDs for the order items in a single query
        let product_ids: Vec<ProductId> = order_items
            .as_ref()
            .iter()
            .map(|item| item.product_id)
            .collect();

        // Step 2: Fetch products by their IDs from the database
        let products: ProductList = ProductIdList::new(product_ids)
            .get_products_by_ids(db)
            .await?;

        let product_map: std::collections::HashMap<ProductId, Product> = products
            .into_inner()
            .into_iter()
            .map(|product| (product.id, product))
            .collect();

        let mut line_items = Vec::with_capacity(order_items.as_ref().len());

        for item in order_items.as_ref().iter() {
            // Look up the product for the current order item
            let product = product_map
                .get(&item.product_id)
                .ok_or_else(|| GlobalError::GlobalStrErr("Product Not Found".to_string()))?;

            // Get or create the Stripe price ID for the product
            let price_id = if let Some(price_id) = product.stripe_price_id.as_ref() {
                price_id.clone()
            } else {
                self.get_price(&product, stripe_currency).await?.id
            };

            // Create the line item
            line_items.push(CreateCheckoutSessionLineItems {
                quantity: Some(*item.quantity.as_ref() as u64),
                price: Some(price_id.to_string()),
                ..Default::default()
            });
        }

        Ok(line_items)
    }

    fn add_params_checkout_session(
        &self,
        line_items: Vec<CreateCheckoutSessionLineItems>,
    ) -> CreateCheckoutSession {
        let mut params = CreateCheckoutSession::new();
        params.mode = Some(CheckoutSessionMode::Payment);
        params.ui_mode = Some(CheckoutSessionUiMode::Hosted);
        params.success_url = Some(&self.settings.stripe_url.stripe_success_url);
        params.cancel_url = Some(&self.settings.stripe_url.stripe_cancel_url);
        params.line_items = Some(line_items);
        params.expand = &["line_items", "line_items.data.price.product"];

        // Enable billing address collection
        params.shipping_address_collection = Some(CreateCheckoutSessionShippingAddressCollection {
            allowed_countries: vec![
                CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Us,
            ],
        });

        // Enable billing address collection
        params.billing_address_collection = Some(CheckoutSessionBillingAddressCollection::Required);

        params
    }

    pub async fn create_checkout_session(
        &self,
        stripe_currency: &StripeCurrency,
        user: &User,
        order_item: &OrderItemList,
        db: &Arc<Database>,
    ) -> Result<CheckoutSession, GlobalError> {
        let customer_id: CustomerId = if let Some(stripe_id) = user.stripe_customer_id.as_ref() {
            stripe_id.clone()
        } else {
            let cust_id = self.get_customer(user).await?.id;
            user.save_stripe_customer_id(&cust_id, db).await?;
            cust_id
        };
        let line_items: Vec<CreateCheckoutSessionLineItems> = self
            .create_line_items(order_item, db, stripe_currency)
            .await?;

        let mut params = self.add_params_checkout_session(line_items);
        params.customer = Some(customer_id);

        Ok(CheckoutSession::create(&self.client, params)
            .await
            .map_err(|e| GlobalError::GlobalStrErr(e.to_string()))?)
    }

    pub async fn create_guest_checkout_session(
        &self,
        stripe_currency: &StripeCurrency,
        customer_id: &CustomerId,
        order_item: &GuestOrderItemList,
        db: &Arc<Database>,
    ) -> Result<CheckoutSession, GlobalError> {
        let line_items = self
            .create_guest_line_items(order_item, db, stripe_currency)
            .await?;

        let mut params = self.add_params_checkout_session(line_items);
        params.customer = Some(customer_id.clone());

        Ok(CheckoutSession::create(&self.client, params)
            .await
            .map_err(|e| GlobalError::GlobalStrErr(e.to_string()))?)
    }

    pub async fn create_payment_link(
        &self,
        product: &Product,
        stripe_currency: &StripeCurrency,
        order_item: &OrderItem,
    ) -> Result<PaymentLink, StripeErr> {
        let price = self.get_price(product, stripe_currency).await?;
        let quantity = *order_item.quantity.as_ref() as u64;

        let payment_link = PaymentLink::create(
            &self.client,
            CreatePaymentLink::new(vec![CreatePaymentLinkLineItems {
                quantity: quantity,
                price: price.id.to_string(),
                ..Default::default()
            }]),
        )
        .await?;

        Ok(payment_link)
    }

    pub async fn create_stripe_connect_link(&self) -> Result<AccountLink, StripeErr> {
        let account = self.create_account_stripe_connect().await?;
        let link = AccountLink::create(
            &self.client,
            CreateAccountLink {
                account: account.id,
                type_: AccountLinkType::AccountOnboarding,
                collect: None,
                expand: &[],
                refresh_url: Some(self.settings.stripe_url.stripe_refresh_url.as_ref()),
                return_url: Some(self.settings.stripe_url.stripe_return_url.as_ref()),
                collection_options: None,
            },
        )
        .await?;
        Ok(link)
    }
}

// Not being used ATM, using MetaEntityType now, but could be useful in the future.
pub trait Metadata {
    fn build_metadata(
        &self,
        base_meta: &std::collections::HashMap<String, String>,
    ) -> std::collections::HashMap<String, String>;
}

impl Metadata for Product {
    fn build_metadata(
        &self,
        base_meta: &std::collections::HashMap<String, String>,
    ) -> std::collections::HashMap<String, String> {
        let mut meta = base_meta.clone();
        meta.insert("product_id".to_string(), self.id.as_ref().to_string());
        meta.insert(
            "category_id".to_string(),
            self.category_id.as_ref().to_string(),
        );
        meta
    }
}

impl Metadata for User {
    fn build_metadata(
        &self,
        base_meta: &std::collections::HashMap<String, String>,
    ) -> std::collections::HashMap<String, String> {
        let mut meta = base_meta.clone();
        meta.insert("user_id".to_string(), self.id.as_ref().to_string());
        meta.insert("email".to_string(), self.email.as_ref().to_string());
        meta
    }
}
