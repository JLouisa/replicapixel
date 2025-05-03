use std::collections::HashMap;

use super::auth::{UserCreditsView, UserView};
use super::images::ImageViewList;
use super::settings::UserSettingsView;
use super::training_models::TrainingModelView;
use crate::domain::features::FeatureViewList;
use crate::domain::website::Website;
use crate::middleware::cookie::CookieConsent;
use crate::models::_entities::sea_orm_active_enums::{PlanNames, Status};
use crate::models::packs::PackModelList;
use crate::models::transactions::TransactionModelList;
use crate::models::{PackModel, PlanModel, TransactionModel};
use derive_more::{AsRef, Constructor};
use loco_rs::prelude::*;
use serde::Serialize;

pub fn billing_dashboard(
    v: impl ViewRenderer,
    website: &Website,
    user: UserView,
    credits: &UserCreditsView,
    orders: &TransactionViewList,
    cc_cookie: &CookieConsent,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "dashboard/content/billing/billing.html",
        data!(
            {
                "website": website, "user": user,
                "credits": credits, "cc_cookie": cc_cookie,
                "orders": orders
            }
        ),
    )
}
pub fn billing_partial_dashboard(
    v: impl ViewRenderer,
    website: &Website,
    user: UserView,
    orders: &TransactionViewList,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "dashboard/content/billing/billing_partial.html",
        data!({ "website": website, "user": user,  "orders": orders }),
    )
}

pub fn notification_dashboard(
    v: impl ViewRenderer,
    website: &Website,
    user: UserView,
    credits: &UserCreditsView,
    cc_cookie: &CookieConsent,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "dashboard/content/notification/notification.html",
        data!(
            {
                "website": website, "user": user,
                "credits": credits, "cc_cookie": cc_cookie
        }),
    )
}
pub fn notification_partial_dashboard(
    v: impl ViewRenderer,
    website: &Website,
    user: UserView,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "dashboard/content/notification/notification_partial.html",
        data!({ "website": website, "user": user }),
    )
}

pub fn features_dashboard(
    v: impl ViewRenderer,
    website: &Website,
    user: UserView,
    credits: &UserCreditsView,
    features_view: &FeatureViewList,
    cc_cookie: &CookieConsent,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "dashboard/content/features/features.html",
        data!(
            {
                "website": website, "user": user, "credits": credits,
                "features_view": features_view, "cc_cookie": cc_cookie
            }
        ),
    )
}
pub fn features_partial_dashboard(
    v: impl ViewRenderer,
    website: &Website,
    user: UserView,
    features_view: &FeatureViewList,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "dashboard/content/features/features_partial.html",
        data!(
            {
                "website": website, "user": user,
                "features_view": features_view,
            }
        ),
    )
}

pub fn settings_dashboard(
    v: impl ViewRenderer,
    website: &Website,
    user: &UserView,
    credits: &UserCreditsView,
    user_settings: &UserSettingsView,
    cc_cookie: &CookieConsent,
    is_oauth: bool,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "dashboard/content/settings/settings.html",
        data!(
            {
                "website": website, "user": user, "credits": credits,
                "cc_cookie": cc_cookie, "user_settings": user_settings,
                "is_oauth": is_oauth
            }
        ),
    )
}
pub fn settings_partial_dashboard(
    v: impl ViewRenderer,
    website: &Website,
    user: &UserView,
    user_settings: &UserSettingsView,
    is_oauth: bool,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "dashboard/content/settings/settings_partial.html",
        data!(
            {
                "website": website, "user": user, "user_settings": user_settings,
                "is_oauth": is_oauth
            }
        ),
    )
}

pub fn training_dashboard(
    v: impl ViewRenderer,
    website: &Website,
    user: &UserView,
    credits: &UserCreditsView,
    models: &Vec<TrainingModelView>,
    cc_cookie: &CookieConsent,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "dashboard/content/training_models/training_models.html",
        data!(
            {
                "website": website, "user": user, "credits": credits,
                "models": models, "cc_cookie": cc_cookie
            }
        ),
    )
}
pub fn training_partial_dashboard(
    v: impl ViewRenderer,
    website: &Website,
    user: &UserView,
    credits: &UserCreditsView,
    models: &Vec<TrainingModelView>,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "dashboard/content/training_models/training_models_partial.html",
        data!({ "website": website, "user": user, "credits": credits, "models": models }),
    )
}
pub fn create_training_dashboard(
    v: impl ViewRenderer,
    website: &Website,
    user: &UserView,
    credits: &UserCreditsView,
    models: &Vec<TrainingModelView>,
    cc_cookie: &CookieConsent,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "dashboard/content/training_models/extend_training_model_form.html",
        data!(
            {
                "website": website, "user": user, "credits": credits,
                "models": models, "cc_cookie": cc_cookie
            }
        ),
    )
}
pub fn create_training_dashboard_partial(
    v: impl ViewRenderer,
    website: &Website,
    user: &UserView,
    credits: &UserCreditsView,
    models: &Vec<TrainingModelView>,
    cc_cookie: &CookieConsent,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "dashboard/content/training_models/training_model_form.html",
        data!(
            {
                "website": website, "user": user, "credits": credits,
                "models": models, "cc_cookie": cc_cookie
            }
        ),
    )
}

pub fn packs_dashboard(
    v: impl ViewRenderer,
    website: &Website,
    user: &UserView,
    credits: &UserCreditsView,
    packs: PackViewList,
    cc_cookie: &CookieConsent,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "dashboard/content/packs/packs.html",
        data!(
            {
                "website": website, "credits": credits, "packs": packs,
                "cc_cookie": cc_cookie, "user": user,
            }
        ),
    )
}
pub fn packs_partial_dashboard(
    v: impl ViewRenderer,
    website: &Website,
    packs: PackViewList,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "dashboard/content/packs/packs_partial.html",
        data!({ "website": website, "packs": packs,  "partial": true }),
    )
}

pub fn photo_dashboard(
    v: impl ViewRenderer,
    website: &Website,
    user: &UserView,
    credits: &UserCreditsView,
    images: &ImageViewList,
    training_models: Vec<TrainingModelView>,
    is_deleted: bool,
    is_favorite: bool,
    cc_cookie: &CookieConsent,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "dashboard/content/photo/photo.html",
        data!(
              {
                  "website": website, "user": user, "images": images,
                  "training_models": training_models, "credits": credits,
                  "is_deleted": is_deleted, "is_favorite": is_favorite,
                  "is_initial_load": true, "cc_cookie": cc_cookie
              }
        ),
    )
}
pub fn photo_partial_dashboard(
    v: impl ViewRenderer,
    website: &Website,
    images: &ImageViewList,
    training_models: Vec<TrainingModelView>,
    credits: &UserCreditsView,
    is_deleted: bool,
    is_favorite: bool,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "dashboard/content/photo/photo_partial.html",
        data!(
              {
                "website": website, "images": images,
                "training_models": training_models,
                "credits": credits, "is_deleted": is_deleted,
                "is_favorite": is_favorite
            }
        ),
    )
}

#[derive(Debug, Serialize, Clone, Constructor)]
pub struct PackView {
    pub id: i32,
    pub pid: Uuid,
    pub title: String,
    pub description: String,
    pub credits: i32,
    pub amount: i32,
    pub image_url: String,
}
impl From<PackModel> for PackView {
    fn from(p: PackModel) -> Self {
        Self {
            id: p.id,
            pid: p.pid,
            title: p.title,
            description: p.description,
            credits: p.credits,
            amount: p.amount,
            image_url: p.image_url,
        }
    }
}

#[derive(Debug, Serialize, Clone, AsRef, Constructor)]
pub struct PackViewList(pub Vec<PackView>);

impl From<PackModelList> for PackViewList {
    fn from(p: PackModelList) -> Self {
        Self(p.0.into_iter().map(|x| x.into()).collect())
    }
}

#[derive(Clone, Debug, Serialize, Constructor)]
pub struct TransactionView {
    pub id: i32,
    pub pid: Uuid,
    pub user_id: i32,
    pub plan: PlanNames,
    pub credit_amount: i32,
    pub model_amount: i32,
    pub currency: String,
    pub status: Status,
    pub created_at: String,
    pub payment_amount: Decimal,
}
impl TransactionView {
    pub fn from_model(t: TransactionModel, p: PlanNames) -> Self {
        Self {
            id: t.id,
            pid: t.pid,
            user_id: t.user_id,
            plan: p,
            credit_amount: t.credit_amount,
            model_amount: t.model_amount,
            currency: t.currency,
            status: t.status,
            created_at: format!(
                "{} (UTC)",
                t.created_at.naive_utc().format("%Y-%m-%d %H:%M")
            ),
            payment_amount: Decimal::new(t.payment_amount, 2),
        }
    }
}

#[derive(Debug, Serialize, Clone, Constructor, AsRef)]
pub struct TransactionViewList(Vec<TransactionView>);
impl TransactionViewList {
    pub fn from_model(t: TransactionModelList, p: HashMap<i32, PlanModel>) -> Self {
        Self(
            t.as_ref()
                .into_iter()
                .cloned()
                .map(|x| {
                    let plan_name = match p.get(&x.plan_id) {
                        Some(p) => PlanNames::from(p.plan_name.clone()),
                        None => {
                            tracing::error!("Transaction has invalid plan_id {}", x.plan_id);
                            PlanNames::Max
                        }
                    };
                    TransactionView::from_model(x, plan_name)
                })
                .collect(),
        )
    }
}
