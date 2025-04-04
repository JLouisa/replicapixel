use serde::{Deserialize, Serialize};

use crate::models::{UserCreditModel, UserModel, _entities::users};

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

// ============== View Models for the View Templates ==============
#[derive(Serialize)]
pub struct UserView {
    pub id: i32,
    pub pid: String,
    pub name: String,
    pub email: String,
}
impl From<&UserModel> for UserView {
    fn from(model: &UserModel) -> Self {
        Self {
            id: model.id,
            pid: model.pid.to_string(),
            name: model.name.clone(),
            email: model.email.clone(),
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
