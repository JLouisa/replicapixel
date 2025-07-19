use derive_more::{AsRef, Constructor};
use loco_rs::prelude::*;
use serde::Serialize;
use std::collections::HashMap;

use crate::domain::website::WebsiteOptions;
use crate::models::_entities::sea_orm_active_enums::{PlanNames, Status};
use crate::models::transactions::TransactionModelList;
use crate::models::{PlanModel, TransactionModel};

pub fn billing_dashboard_new(
    v: impl ViewRenderer,
    website_options: &WebsiteOptions,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "dashboard/content/billing/billing.html",
        data!({ "options": website_options }),
    )
}
pub fn billing_partial_dashboard_new(
    v: impl ViewRenderer,
    website_options: &WebsiteOptions,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "dashboard/content/billing/billing_partial.html",
        data!({ "options": website_options }),
    )
}

pub fn features_dashboard(
    v: impl ViewRenderer,
    website_options: &WebsiteOptions,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "dashboard/content/features/features.html",
        data!({ "options": website_options }),
    )
}
pub fn features_partial_dashboard(
    v: impl ViewRenderer,
    website_options: &WebsiteOptions,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "dashboard/content/features/features_partial.html",
        data!({ "options": website_options }),
    )
}

pub fn settings_dashboard(
    v: impl ViewRenderer,
    website_options: &WebsiteOptions,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "dashboard/content/settings/settings.html",
        data!({ "options": website_options }),
    )
}
pub fn settings_partial_dashboard(
    v: impl ViewRenderer,
    website_options: &WebsiteOptions,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "dashboard/content/settings/settings_partial.html",
        data!({ "options": website_options }),
    )
}

pub fn training_dashboard(
    v: impl ViewRenderer,
    website_options: &WebsiteOptions,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "dashboard/content/training_models/training_models.html",
        data!({ "options": website_options }),
    )
}
pub fn training_partial_dashboard(
    v: impl ViewRenderer,
    website_options: &WebsiteOptions,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "dashboard/content/training_models/training_models_partial.html",
        data!({ "options": website_options }),
    )
}

pub fn create_training_dashboard(
    v: impl ViewRenderer,
    website_options: &WebsiteOptions,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "dashboard/content/training_models/extend_training_model_form.html",
        data!({ "options": website_options }),
    )
}
pub fn create_training_dashboard_partial(
    v: impl ViewRenderer,
    website_options: &WebsiteOptions,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "dashboard/content/training_models/training_model_form.html",
        data!({ "options": website_options }),
    )
}

pub fn home_training_partial_dashboard(
    v: impl ViewRenderer,
    website_options: &WebsiteOptions,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "dashboard/dashboard_base_extend_partial.html",
        data!({ "options": website_options }),
    )
}

pub fn packs_dashboard(
    v: impl ViewRenderer,
    website_options: &WebsiteOptions,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "dashboard/content/packs/packs.html",
        data!({ "options": website_options }),
    )
}
pub fn packs_partial_dashboard(
    v: impl ViewRenderer,
    website_options: &WebsiteOptions,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "dashboard/content/packs/packs_partial.html",
        data!({ "options": website_options }),
    )
}
pub fn home_packs_partial_dashboard(
    v: impl ViewRenderer,
    website_options: &WebsiteOptions,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "dashboard/content/packs/packs_extend.html",
        data!({ "options": website_options }),
    )
}

pub fn photo_dashboard(
    v: impl ViewRenderer,
    website_options: &WebsiteOptions,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "dashboard/content/photo/photo.html",
        data!({ "options": website_options }),
    )
}
pub fn photo_partial_dashboard(
    v: impl ViewRenderer,
    website_options: &WebsiteOptions,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "dashboard/content/photo/photo_partial.html",
        data!({ "options": website_options }),
    )
}

pub fn video_dashboard(
    v: impl ViewRenderer,
    website_options: &WebsiteOptions,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "dashboard/content/video/video.html",
        data!({ "options": website_options }),
    )
}
pub fn video_partial_dashboard(
    v: impl ViewRenderer,
    website_options: &WebsiteOptions,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "dashboard/content/video/video_partial.html",
        data!({ "options": website_options }),
    )
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
