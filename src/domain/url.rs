use std::str::FromStr;

use derive_more::{Display, From};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use validator::ValidateUrl;

#[derive(Error, Debug)]
pub enum UrlError {
    #[error("S3 error: {0}")]
    UrlError(#[from] validator::ValidationError),
    #[error("Other error: {0}")]
    Other(String),
}

#[derive(Debug, Serialize, Deserialize, Display, From)]
pub struct Url(String);

impl Url {
    pub fn new<K: Into<String>>(key: K) -> Self {
        Self(key.into())
    }
    pub fn into_inner(self) -> String {
        self.0
    }
    pub fn validate(url: &str) -> Result<Self, UrlError> {
        // Trim the URL
        let url = url.trim();

        // Check if the URL is empty or whitespace
        let is_empty_or_whitespace = url.is_empty();
        dbg!(&is_empty_or_whitespace);

        // Check if the URL is valid
        let valid_url = ValidateUrl::validate_url(url);
        dbg!(&valid_url);

        // Check if the URL is valid
        if is_empty_or_whitespace || !valid_url {
            Err(UrlError::Other("Invalid URL".to_string()))
        } else {
            Ok(Self(url.to_owned()))
        }
    }
    pub fn validate_many(urls: &Vec<String>) -> Result<Vec<Self>, UrlError> {
        urls.iter()
            .map(|url| Self::validate(url))
            .collect::<Result<Vec<Self>, UrlError>>()
    }

    pub fn many_to_string(urls: &Vec<Url>) -> Vec<String> {
        urls.iter().map(|url| url.as_ref().to_owned()).collect()
    }
}

impl AsRef<String> for Url {
    fn as_ref(&self) -> &String {
        &self.0
    }
}
