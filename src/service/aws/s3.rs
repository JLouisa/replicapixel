use aws_sdk_s3 as s3;
use aws_sdk_s3::config::BehaviorVersion;
use aws_sdk_s3::operation::delete_object::DeleteObjectError;
use aws_sdk_s3::operation::head_object::HeadObjectError;
use cuid2;
use s3::config::{Credentials, Region};
use s3::error::SdkError;
use s3::operation::put_object::PutObjectError;
use s3::presigning::{PresigningConfig, PresigningConfigError};
use s3::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use thiserror::Error;
use uuid::Uuid;

use crate::domain::url::Url;
use crate::models::_entities::sea_orm_active_enums::ImageFormat;
use crate::models::training_models::TrainingForm;
use crate::views::images::ImageView;

#[derive(Error, Debug)]
pub enum AwsError {
    #[error("S3 error: {0}")]
    S3Err(#[from] s3::Error),
    #[error("PresigningConfigError error: {0}")]
    S3PresigningConfigErr(#[from] PresigningConfigError),
    #[error("PutObjectError error: {0}")]
    PutObjectErr(#[from] SdkError<PutObjectError>),
    #[error("HeadObjectError error: {0}")]
    HeadObjectErr(#[from] SdkError<HeadObjectError>),
    #[error("S3 Deletion error: {0}")]
    S3DeletionError(#[from] SdkError<DeleteObjectError>),
    #[error("Other error: {0}")]
    Other(String),
}

#[derive(Error, Debug)]
pub enum OtherError {
    #[error("S3 error: {0}")]
    UrlError(#[from] validator::ValidationError),
    #[error("Other error: {0}")]
    Other(String),
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct AwsSettings {
    pub access_key_id: String,
    pub secret_access_key: String,
    pub s3: S3,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct S3 {
    pub region: String,
    pub bucket_name: String,
    pub access_time: u64,
}

#[derive(Serialize, Deserialize)]
pub struct PresignedUrlRequestForm {
    pub name: String,
    pub file_type: ImageFormat,
}

#[derive(Serialize, Deserialize)]
pub struct PresignedUrlRequest {
    pub id: Uuid,
    pub name: String,
    pub file_type: ImageFormat,
}
impl From<TrainingForm> for PresignedUrlRequest {
    fn from(value: TrainingForm) -> Self {
        Self {
            id: value.pid,
            name: format!("{}-{}", value.name, value.slug),
            file_type: value.file_type,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct S3Key(String);
impl S3Key {
    pub fn new<K: Into<String>>(key: K) -> Self {
        Self(key.into())
    }
    pub fn into_inner(self) -> String {
        self.0
    }
    pub fn full_url(&self, bucket_name: &str) -> String {
        format!("https://{}.s3.amazonaws.com/{}", bucket_name, self.0)
    }
}
impl AsRef<str> for S3Key {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
impl Default for S3Key {
    fn default() -> Self {
        Self(String::new())
    }
}

impl From<PresignedUrlRequestForm> for PresignedUrlRequest {
    fn from(value: PresignedUrlRequestForm) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: format!("{}-{}", value.name, cuid2::slug()),
            file_type: value.file_type,
        }
    }
}

//Todo Re-enable
// impl From<TrainingForm> for PresignedUrlRequest {
//     fn from(value: TrainingForm) -> Self {
//         Self {
//             id: value.pid,
//             name: format!("{}-{}", value.name, value.slug),
//             file_type: value.file_type,
//         }
//     }
// }

#[derive(Serialize, Deserialize)]
pub struct PresignedUrlSafe {
    pub id: Uuid,
    pub name: String,
    pub file_type: ImageFormat,
    pub pre_url: Url,
}

impl PresignedUrlSafe {
    pub fn from_request(value: PresignedUrlRequest, pre_url: Url) -> Self {
        Self {
            id: value.id,
            name: value.name,
            file_type: value.file_type,
            pre_url,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AwsS3 {
    pub client: Client,
    pub settings: AwsSettings,
}
impl AwsS3 {
    pub fn bucket_name(&self) -> &String {
        &self.settings.s3.bucket_name
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum S3Folders {
    Images,
    Zip,
    Website,
    Documents,
}
impl S3Folders {
    pub fn get_folder_str(&self) -> String {
        match self {
            S3Folders::Images => "images".to_string(),
            S3Folders::Zip => "zip".to_string(),
            S3Folders::Website => "website".to_string(),
            S3Folders::Documents => "documents".to_string(),
        }
    }
}

impl AwsS3 {
    pub async fn new(settings: &AwsSettings) -> Self {
        let credentials = Credentials::new(
            &settings.access_key_id,
            &settings.secret_access_key,
            None,
            None,
            "manual",
        );

        // Ensure the region is correctly initialized
        let region = settings.s3.region.to_string();
        // dbg!(&region); // Debug to ensure the region is correctly set

        let aws_config = aws_sdk_s3::Config::builder()
            .behavior_version(BehaviorVersion::v2025_01_17())
            .region(Region::new(region))
            .credentials_provider(credentials)
            .build();

        let s3_client = s3::Client::from_conf(aws_config);
        Self {
            client: s3_client,
            settings: settings.clone(),
        }
    }

    pub fn create_byte_data(&self, data: &str) -> Vec<u8> {
        data.as_bytes().to_vec()
    }

    // Generate a presigned URL
    pub async fn auto_upload_img_presigned_url(&self, image: &ImageView) -> Result<Url, AwsError> {
        let key = S3Key::new(image.image_s3_key.to_owned());
        let time = Some(300);
        let pre_url = self.generate_save_presigned_url(&key, time).await?;
        Ok(pre_url)
    }

    //Todo HERE Generate a presigned URL
    pub async fn presigned_save_url(
        &self,
        user_id: &Uuid,
        url_request: &PresignedUrlRequest,
        time: Option<u64>,
    ) -> Result<(Url, S3Key), AwsError> {
        let folder = match url_request.file_type {
            ImageFormat::Zip => S3Folders::Zip,
            ImageFormat::Jpeg => S3Folders::Images,
            ImageFormat::Png => S3Folders::Images,
        };
        let key = self.create_s3_key(user_id, &folder, &url_request.name, &url_request.file_type);
        let pre_url = self.generate_save_presigned_url(&key, time).await?;

        dbg!(&key);
        Ok((pre_url, key))
    }

    pub async fn generate_save_presigned_url(
        &self,
        key: &S3Key,
        time: Option<u64>,
    ) -> Result<Url, AwsError> {
        let time = match time {
            Some(t) => t,
            None => self.settings.s3.access_time,
        };

        let presigned_req = self
            .client
            .put_object()
            .bucket(&self.settings.s3.bucket_name)
            .key(key.as_ref())
            .presigned(PresigningConfig::expires_in(Duration::from_secs(time)).unwrap())
            .await
            .expect("Failed to generate presigned URL");

        let pre_url = Url::new(presigned_req.uri().to_string());
        Ok(pre_url)
    }

    // Check if an object exists in the S3 bucket
    pub async fn check_object_exists(&self, s3_key: &S3Key) -> Result<bool, AwsError> {
        match self
            .client
            .head_object()
            .bucket(&self.settings.s3.bucket_name)
            .key(s3_key.as_ref())
            .send()
            .await
        {
            Ok(_) => Ok(true), // If the request succeeds, the object exists
            Err(SdkError::ServiceError(service_error)) if service_error.err().is_not_found() => {
                Ok(false)
            } // Object does not exist
            Err(e) => Err(AwsError::HeadObjectErr(e)), // Wrap the error in Err
        }
    }

    // Get a presigned URL for an object
    pub async fn get_object_pre(&self, key: &S3Key, time: Option<u64>) -> Result<Url, AwsError> {
        let expires_in = match time {
            Some(t) => t,
            None => self.settings.s3.access_time * 24, // 1 hour * 24 | 1 day
        };
        let presigned_request = self
            .client
            .get_object()
            .bucket(&self.settings.s3.bucket_name)
            .key(key.as_ref())
            .presigned(PresigningConfig::expires_in(Duration::from_secs(
                expires_in,
            ))?)
            .await
            .map_err(|_| AwsError::Other("Error getting presigned URL: 101".to_string()))?;
        Ok(Url::new(presigned_request.uri().to_string()))
    }

    pub async fn get_object_pre_many() -> Result<Vec<Url>, AwsError> {
        todo!()
    }

    /// Delete an object from a bucket.
    pub async fn remove_object(
        &self,
        user_id: &Uuid,
        folder: &S3Folders,
        item_name: &str,
    ) -> Result<(), AwsError> {
        let s3_location_key = self.create_item_path(&user_id, &folder, &item_name);

        match self
            .client
            .delete_object()
            .bucket(&self.settings.s3.bucket_name)
            .key(s3_location_key)
            .send()
            .await
        {
            Ok(_) => Ok(()), // If the request succeeds, the object exists
            Err(e) => Err(AwsError::S3DeletionError(e)), // Wrap the error in Err
        }
    }

    /// Delete an object from a bucket.
    pub async fn remove_object_s3_key(&self, key: &S3Key) -> Result<(), AwsError> {
        match self
            .client
            .delete_object()
            .bucket(&self.settings.s3.bucket_name)
            .key(key.as_ref())
            .send()
            .await
        {
            Ok(_) => Ok(()), // If the request succeeds, the object exists
            Err(e) => Err(AwsError::S3DeletionError(e)), // Wrap the error in Err
        }
    }

    pub fn create_full_path(&self, key: &S3Key) -> Url {
        Url::new(format!(
            "https://{}.s3.{}.amazonaws.com/{}",
            self.settings.s3.bucket_name,
            self.settings.s3.region,
            key.as_ref()
        ))
    }

    pub fn create_item_path(&self, id: &Uuid, folder: &S3Folders, item_name: &str) -> String {
        format!(
            "{}/{}/{}",
            id.to_string(),
            folder.get_folder_str(),
            item_name.to_string()
        )
    }

    pub fn create_s3_key(
        &self,
        id: &Uuid,
        folder: &S3Folders,
        item_name: &str,
        file_format: &ImageFormat,
    ) -> S3Key {
        let key = format!(
            "{}/{}/{}.{}",
            id.to_string(),
            folder.get_folder_str(),
            item_name.to_string(),
            file_format.to_string()
        );
        S3Key::new(key)
    }

    pub fn init_img_s3_key(user_id: &Uuid, img_id: &Uuid) -> S3Key {
        let key = format!(
            "{}/{}/{}.{}",
            user_id,
            S3Folders::Images.get_folder_str(),
            img_id,
            ImageFormat::Jpeg.to_string()
        );
        S3Key::new(key)
    }

    pub fn create_s3_key_img(&self, id: &Uuid, image_name: &Uuid) -> S3Key {
        let key = format!(
            "{}/{}/{}.{}",
            id.to_string(),
            S3Folders::Images.get_folder_str(),
            image_name.to_string(),
            ImageFormat::Jpeg,
        );
        S3Key::new(key)
    }
}
