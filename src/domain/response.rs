use axum::response::Response;
use axum::{http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::cookie::CookieJar;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::marker::PhantomData;
use tracing::error;

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneralResponseMsg {
    pub status: ResponseStatus,
    pub message: Option<String>,
    pub data: Option<String>,
}
impl GeneralResponseMsg {
    pub fn new(status: ResponseStatus, message: Option<String>, data: Option<String>) -> Self {
        Self {
            status,
            message,
            data,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseMsg<T: ClientOnly> {
    pub status: ResponseStatus,
    pub message: Option<String>,
    pub data: Option<T>,
    #[serde(skip)]
    pub _marker: PhantomData<T>,
}

impl<T: ClientOnly> ResponseMsg<T> {
    pub fn success(data: T) -> Self {
        Self {
            status: ResponseStatus::Success,
            message: None,
            data: Some(data),
            _marker: PhantomData,
        }
    }

    pub fn error(msg: ErrorMessage) -> Self {
        Self {
            status: ResponseStatus::Error,
            message: Some(msg.to_string().to_string()),
            data: None,
            _marker: PhantomData,
        }
    }

    pub fn general(status: ResponseStatus, message: Option<String>, data: Option<T>) -> Self {
        Self {
            status,
            message,
            data,
            _marker: PhantomData,
        }
    }
}

pub fn handle_general_response<T>(
    status_code: StatusCode,
    data: Option<T>,
    message: Option<String>,
) -> Response
where
    T: Serialize + ClientOnly,
{
    let resp_status: ResponseStatus = match status_code {
        StatusCode::OK => ResponseStatus::Success,
        _ => ResponseStatus::Error,
    };
    let response = ResponseMsg::general(resp_status, message, data);
    (status_code, Json(serde_json::json!(response))).into_response()
}

pub fn handle_general_response_with_cookie<T>(
    status_code: StatusCode,
    data: Option<T>,
    message: Option<String>,
    cookie_jar: CookieJar,
) -> Response
where
    T: Serialize,
{
    let resp_status: ResponseStatus = match status_code {
        StatusCode::OK => ResponseStatus::Success,
        _ => ResponseStatus::Error,
    };
    let response = ResponseMsg::general(resp_status, message, data);
    (status_code, cookie_jar, Json(response)).into_response()
}

pub fn handle_general_response_text(
    status_code: StatusCode,
    data: Option<String>,
    message: Option<String>,
) -> impl IntoResponse {
    let resp_status: ResponseStatus = match status_code {
        StatusCode::OK => ResponseStatus::Success,
        _ => ResponseStatus::Error,
    };
    let response: GeneralResponseMsg = GeneralResponseMsg::new(resp_status, message, data);
    (status_code, Json(serde_json::json!(response))).into_response()
}

pub fn handle_db_response<T>(
    result: Result<T, impl std::fmt::Debug>,
    success_status: StatusCode,
    error_message: ErrorMessage,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)>
where
    T: Serialize + ClientOnly,
{
    match result {
        Ok(data) => {
            let response = ResponseMsg::success(data);
            Ok((success_status, Json(serde_json::json!(response))))
        }
        Err(e) => {
            error!("Database operation failed: {:?}", e);
            let response: ResponseMsg<()> = ResponseMsg::error(error_message);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!(response)),
            ))
        }
    }
}

pub fn handle_optional_db_response<T>(
    result: Result<Option<T>, impl Debug>,
    success_status: StatusCode,
    error_message: ErrorMessage,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)>
where
    T: Serialize + ClientOnly,
{
    match result {
        Ok(Some(data)) => {
            let response = ResponseMsg::success(data);
            Ok((success_status, Json(serde_json::json!(response))))
        }
        Ok(None) => {
            let response: ResponseMsg<()> = ResponseMsg::error(ErrorMessage::NotFound);
            Err((StatusCode::NOT_FOUND, Json(serde_json::json!(response))))
        }
        Err(e) => {
            error!("Database operation failed: {:?}", e);
            let response: ResponseMsg<()> = ResponseMsg::error(error_message);
            Err((StatusCode::BAD_GATEWAY, Json(serde_json::json!(response))))
        }
    }
}

pub trait ClientOnly: Serialize {}
impl<T: Serialize> ClientOnly for T {}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ResponseStatus {
    Success,
    Error,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ErrorMessage {
    InternalServerError,
    BadRequest,
    NotFound,
    Unauthorized,
    Forbidden,
    ValidationFailed,
    Conflict,
    ServiceUnavailable,
    GatewayTimeout,
    InvalidCouponCode,
    InvalidLogin,
    UserNotActive,
}

impl ErrorMessage {
    pub fn to_string(&self) -> &str {
        match self {
            Self::InternalServerError => "An unexpected error occurred. Please try again later.",
            Self::BadRequest => {
                "The request could not be understood or was missing required parameters."
            }
            Self::NotFound => "The requested resource could not be found.",
            Self::Unauthorized => "You are not authorized to perform this action. Please log in.",
            Self::Forbidden => "You do not have permission to access this resource.",
            Self::ValidationFailed => {
                "One or more fields failed validation. Please check your input and try again."
            }
            Self::Conflict => "There was a conflict with the current state of the resource.",
            Self::ServiceUnavailable => {
                "The service is temporarily unavailable. Please try again later."
            }
            Self::GatewayTimeout => "The server took too long to respond. Please try again later.",
            Self::InvalidCouponCode => "The coupon code is invalid.",
            Self::InvalidLogin => "Invalid login",
            Self::UserNotActive => "User is not active",
        }
    }
}
