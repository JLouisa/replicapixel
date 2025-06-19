use axum::response::IntoResponse;
use loco_rs::prelude::*;

use loco_rs::prelude::ViewRenderer;
use serde::{Deserialize, Serialize};

use crate::{
    domain::website::{Website, WebsiteOptions},
    models::{UserCreditModel, UserModel, _entities::sea_orm_active_enums::Account},
};

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub pid: String,
    pub name: String,
    pub is_verified: bool,
}

impl LoginResponse {
    #[must_use]
    pub fn new(user: &UserModel, token: &String) -> Self {
        Self {
            token: token.to_string(),
            pid: user.pid.to_string(),
            name: user.name.clone(),
            is_verified: user.email_verified_at.is_some(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CurrentResponse {
    pub pid: String,
    pub name: String,
    pub email: String,
}

impl CurrentResponse {
    #[must_use]
    pub fn new(user: &UserModel) -> Self {
        Self {
            pid: user.pid.to_string(),
            name: user.name.clone(),
            email: user.email.clone(),
        }
    }
}

pub fn login(v: impl ViewRenderer, website: &Website) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "auth/login/login_form.html",
        data!({"options": WebsiteOptions::new().website(&website)}),
    )
}

pub fn partial_login(v: impl ViewRenderer, website: &Website) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "auth/login/login_partial.html",
        data!({"options": WebsiteOptions::new().website(&website)}),
    )
}

pub fn forgot(
    v: &impl ViewRenderer,
    website_options: &WebsiteOptions,
) -> Result<impl IntoResponse> {
    return format::render().view(
        v,
        "auth/forgot/partials/forgot_msg.html",
        data!({"options": website_options}),
    );
}

// ============== View Models for the View Templates ==============
#[derive(Serialize, Clone)]
pub struct UserView {
    pub id: i32,
    pub pid: String,
    pub name: String,
    pub email: String,
    pub account: Account,
}
impl From<&UserModel> for UserView {
    fn from(model: &UserModel) -> Self {
        Self {
            id: model.id,
            pid: model.pid.to_string(),
            name: model.name.clone(),
            email: model.email.clone(),
            account: model.account.clone(),
        }
    }
}
impl From<UserModel> for UserView {
    fn from(model: UserModel) -> Self {
        Self {
            id: model.id,
            pid: model.pid.to_string(),
            name: model.name,
            email: model.email,
            account: model.account,
        }
    }
}

#[derive(Serialize)]
pub struct UserCreditsView {
    pub credit_amount: i32,
    pub model_amount: i32,
}
impl From<&UserCreditModel> for UserCreditsView {
    fn from(model: &UserCreditModel) -> Self {
        Self {
            credit_amount: model.credit_amount,
            model_amount: model.model_amount,
        }
    }
}
impl From<UserCreditModel> for UserCreditsView {
    fn from(model: UserCreditModel) -> Self {
        Self {
            credit_amount: model.credit_amount,
            model_amount: model.model_amount,
        }
    }
}
