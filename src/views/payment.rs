use derive_more::Constructor;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    domain::website::WebsiteOptions,
    models::{
        PlanModel,
        _entities::sea_orm_active_enums::{Currency, PlanNames},
        plans::{PlanDomain, PlanDomainList, PlanModelList},
    },
};

pub fn payment_home_partial(
    v: impl ViewRenderer,
    website_options: &WebsiteOptions,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "payment/payment_plans/payment_plan_partial.html",
        data!({ "options": website_options }),
    )
}

pub fn payment_home(
    v: impl ViewRenderer,
    website_options: &WebsiteOptions,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "payment/payment_plans/payment_plan.html",
        data!({ "options": website_options }),
    )
}

pub fn prepare(
    v: impl ViewRenderer,
    website_options: &WebsiteOptions,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "base_stripe.html",
        data!({ "options": website_options }),
    )
}

pub fn stripe_status(
    v: impl ViewRenderer,
    website_options: &WebsiteOptions,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "payment/stripe/stripe.html",
        data!({ "options": website_options }),
    )
}

pub fn stripe_status_partials(
    v: impl ViewRenderer,
    website_options: &WebsiteOptions,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "payment/stripe/stripe_partials.html",
        data!({ "options": website_options }),
    )
}

#[derive(Debug, Serialize, Deserialize, Constructor, Clone)]
pub struct Feature(String);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PricingView {
    pub pid: Uuid,
    pub plan_name: PlanNames,
    pub credit_amount: i32,
    pub model_amount: i32,
    pub subtitle: String,
    pub price: f64,
    pub currency: Currency,
    pub features: Option<Vec<Feature>>,
    pub cta: String,
    pub is_popular: bool,
}
impl From<PlanModel> for PricingView {
    fn from(plan: PlanModel) -> Self {
        let feature = match plan.features {
            Some(f) => {
                let features: Vec<Feature> = f.iter().map(|f| Feature::new(f.to_owned())).collect();
                Some(features)
            }
            None => None,
        };
        Self {
            pid: plan.pid,
            plan_name: plan.plan_name,
            subtitle: plan.subtitle,
            credit_amount: plan.credit_amount,
            model_amount: plan.model_amount,
            currency: Currency::default(),
            price: plan.price_cents as f64 / 100.0,
            features: feature,
            cta: plan.cta,
            is_popular: plan.is_popular,
        }
    }
}
impl From<PlanDomain> for PricingView {
    fn from(plan: PlanDomain) -> Self {
        let feature = match plan.features {
            Some(f) => {
                let features: Vec<Feature> = f.iter().map(|f| Feature::new(f.to_owned())).collect();
                Some(features)
            }
            None => None,
        };
        Self {
            pid: plan.pid,
            plan_name: plan.plan_name,
            subtitle: plan.subtitle,
            credit_amount: plan.credit_amount,
            model_amount: plan.model_amount,
            currency: Currency::default(),
            price: plan.price_cents as f64 / 100.0,
            features: feature,
            cta: plan.cta,
            is_popular: plan.is_popular,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PricingViewList(Vec<PricingView>);
impl From<PlanModelList> for PricingViewList {
    fn from(mut list: PlanModelList) -> PricingViewList {
        list.0.sort_by_key(|p| p.id);
        Self(list.0.into_iter().map(PricingView::from).collect())
    }
}
impl From<PlanDomainList> for PricingViewList {
    fn from(mut list: PlanDomainList) -> Self {
        list.0.sort_by_key(|p| p.id);
        Self(list.0.into_iter().map(PricingView::from).collect())
    }
}
impl PricingViewList {
    pub fn mock_plans() -> Self {
        let list = vec![
            PricingView::basic(),
            PricingView::premium(),
            PricingView::max(),
        ];

        Self(list)
    }
}
impl Default for PricingViewList {
    fn default() -> Self {
        Self(vec![])
    }
}
impl PricingView {
    pub fn mock_plans() -> PricingViewList {
        let list = vec![
            PricingView::basic(),
            PricingView::premium(),
            PricingView::max(),
        ];

        PricingViewList(list)
    }
    pub fn basic() -> Self {
        Self {
            pid: Uuid::parse_str("cd08b105-5880-4fd1-872a-acf711a5b8ef").unwrap(),
            plan_name: PlanNames::Basic,
            price: 9.99,
            credit_amount: 50,
            model_amount: 1,
            currency: Currency::default(),
            subtitle: "For individuals & testing".to_owned(),
            features: Some(vec![
                Feature::new("No monthly subscription!".to_owned()),
                Feature::new("Use any photo pack".to_owned()),
                Feature::new("No Watermarked photos".to_owned()),
                Feature::new("24/7 Support".to_owned()),
            ]),
            cta: "Choose Basic".to_owned(),
            is_popular: false,
        }
    }
    pub fn premium() -> Self {
        Self {
            pid: Uuid::parse_str("af12e69f-f7e6-4628-b2bd-41ca3489d3af").unwrap(),
            plan_name: PlanNames::Premium,
            price: 39.99,
            credit_amount: 250,
            model_amount: 7,
            currency: Currency::default(),
            subtitle: "For creators & small teams".to_owned(),
            features: Some(vec![
                Feature::new("No monthly subscription!".to_owned()),
                Feature::new("Use any photo pack".to_owned()),
                Feature::new("No Watermarked photos".to_owned()),
                Feature::new("24/7 Support".to_owned()),
            ]),
            cta: "Choose Premium".to_owned(),
            is_popular: true,
        }
    }
    pub fn max() -> Self {
        Self {
            pid: Uuid::parse_str("cd1c6ed7-7a24-4b53-840b-23c81bcc0f4c").unwrap(),
            plan_name: PlanNames::Max,
            price: 99.99,
            credit_amount: 1100,
            model_amount: 16,
            currency: Currency::default(),
            subtitle: "For agencies & heavy users".to_owned(),
            features: Some(vec![
                Feature::new("No monthly subscription!".to_owned()),
                Feature::new("Use any photo pack".to_owned()),
                Feature::new("No Watermarked photos".to_owned()),
                Feature::new("24/7 Support".to_owned()),
            ]),
            cta: "Choose Max".to_owned(),
            is_popular: false,
        }
    }
}
