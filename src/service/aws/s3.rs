use async_trait::async_trait;
use aws_sdk_s3 as s3;
use aws_sdk_s3::config::BehaviorVersion;
use aws_sdk_s3::operation::delete_object::DeleteObjectError;
use aws_sdk_s3::operation::head_object::HeadObjectError;
use aws_sdk_s3::primitives::ByteStream;
use bytes::Bytes;
use cuid2;
use derive_more::Constructor;
use loco_rs::storage::drivers::{GetResponse, StoreDriver, UploadResponse};
use loco_rs::storage::{StorageError, StorageResult};
use s3::config::{Credentials, Region};
use s3::error::SdkError;
use s3::operation::put_object::PutObjectError;
use s3::presigning::{PresigningConfig, PresigningConfigError};
use s3::Client;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::time::Duration;
use thiserror::Error;
use tokio::join;
use uuid::Uuid;

use crate::domain::domain_services::video_generation::VideoAndImageBytes;
use crate::domain::url::Url;
use crate::models::VideoModel;
use crate::models::_entities::sea_orm_active_enums::ImageFormat;
use crate::models::training_models::TrainingFormParam;
use crate::views::images::ImageView;
use crate::views::videos::VideoView;

#[derive(Error, Debug)]
pub enum AwsError {
    #[error("S3 error: {0}")]
    S3Err(#[from] s3::Error),
    #[error("PresigningConfigError error: {0}")]
    LocoError(#[from] PresigningConfigError),
    #[error("PutObjectError error: {0}")]
    PutRequest(#[from] SdkError<PutObjectError>),
    #[error("HeadObjectError error: {0}")]
    RequestFailed(#[from] SdkError<HeadObjectError>),
    #[error("S3 Deletion error: {0}")]
    S3DeletionError(#[from] SdkError<DeleteObjectError>),
    #[error("Other error: {0}")]
    Other(String),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum S3Folders {
    Images,
    Video,
    Thumbnails,
    Zip,
    Website,
    Documents,
}
impl S3Folders {
    pub fn get_folder_str(&self) -> String {
        match self {
            S3Folders::Images => "images".to_string(),
            S3Folders::Video => "video".to_string(),
            S3Folders::Thumbnails => "thumbnails".to_string(),
            S3Folders::Zip => "zip".to_string(),
            S3Folders::Website => "website".to_string(),
            S3Folders::Documents => "documents".to_string(),
        }
    }
    pub fn get_file_type(&self) -> String {
        match self {
            S3Folders::Images => ".jpeg".to_string(),
            S3Folders::Video => ".mp4".to_string(),
            S3Folders::Thumbnails => ".jpeg".to_string(),
            S3Folders::Zip => ".zip".to_string(),
            S3Folders::Website => ".html".to_string(),
            S3Folders::Documents => ".pdf".to_string(),
        }
    }
}

#[derive(Debug, Clone, Constructor)]
pub struct AwsS3Response {
    pub e_tag: Option<String>,
    pub version: Option<String>,
}
impl From<AwsS3Response> for UploadResponse {
    fn from(value: AwsS3Response) -> UploadResponse {
        Self {
            e_tag: value.e_tag,
            version: value.version,
        }
    }
}
impl From<&AwsS3Response> for UploadResponse {
    fn from(value: &AwsS3Response) -> UploadResponse {
        Self {
            e_tag: value.e_tag.clone(),
            version: value.version.clone(),
        }
    }
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
impl From<TrainingFormParam> for PresignedUrlRequest {
    fn from(value: TrainingFormParam) -> Self {
        Self {
            id: value.pid,
            name: format!("{}-{}", value.name, value.slug),
            file_type: value.file_type,
        }
    }
}
impl From<&TrainingFormParam> for PresignedUrlRequest {
    fn from(value: &TrainingFormParam) -> Self {
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
    pub fn bucket_name(&self) -> &String {
        &self.settings.s3.bucket_name
    }
    pub fn region(&self) -> &String {
        &self.settings.s3.region
    }
    pub async fn direct_upload_all(
        &self,
        video_model: &VideoModel,
        vid_and_img: VideoAndImageBytes,
    ) -> Result<(), AwsError> {
        self.direct_upload_video(&video_model, vid_and_img.video_bytes)
            .await?;
        self.direct_upload_image(&video_model, vid_and_img.image_bytes)
            .await?;
        Ok(())
    }
    pub async fn direct_upload_video(
        &self,
        video_model: &VideoModel,
        video_bytes: Bytes,
    ) -> Result<(), AwsError> {
        let key = &video_model.video_s3_key;
        let content_type = "video/mp4";
        self.direct_upload_base(&key, video_bytes.into(), content_type)
            .await?;
        Ok(())
    }
    pub async fn direct_upload_image(
        &self,
        video_model: &VideoModel,
        image_bytes: Vec<u8>,
    ) -> Result<(), AwsError> {
        let key = &video_model.thumbnail_s3_key;
        let content_type = "image/jpeg";
        self.direct_upload_base(&key, image_bytes.into(), content_type)
            .await?;
        Ok(())
    }
    async fn direct_upload_base(
        &self,
        key: &str,
        bytes: ByteStream,
        _content_type: &str,
    ) -> Result<AwsS3Response, AwsError> {
        let response = self
            .client
            .put_object()
            .bucket(&self.settings.s3.bucket_name)
            .key(key)
            .body(bytes)
            // .content_type(content_type)
            .send()
            .await?;
        let response = AwsS3Response::new(response.e_tag, response.version_id);
        Ok(response)
    }
    // Generate a presigned URL
    pub async fn auto_upload_img_presigned_url(&self, image: &ImageView) -> Result<Url, AwsError> {
        let key = S3Key::new(image.image_s3_key.to_owned());
        let time = Some(300);
        let pre_url = self.generate_save_presigned_url(&key, time).await?;
        Ok(pre_url)
    }
    pub async fn auto_upload_presigned_url(&self, key: &S3Key) -> Result<Url, AwsError> {
        let time = Some(300);
        let pre_url = self.generate_save_presigned_url(&key, time).await?;
        Ok(pre_url)
    }
    pub async fn video_save_pre_url(&self, video: VideoModel) -> VideoView {
        let video_key = S3Key::new(&video.video_s3_key);
        let thumbnail_key = S3Key::new(&video.thumbnail_s3_key);

        let (url, url2) = join!(
            self.auto_upload_presigned_url(&video_key),
            self.auto_upload_presigned_url(&thumbnail_key)
        );

        let url = match url {
            Ok(url) => Some(url.to_string()),
            Err(_) => None,
        };
        let url2 = match url2 {
            Ok(url) => Some(url.to_string()),
            Err(_) => None,
        };

        let video: VideoView = VideoView::from(&video)
            .set_video_pre_url(url)
            .set_thumbnail_url(url2);

        video
    }

    // //Todo HERE Generate a presigned URL
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

    pub async fn admin_save_pack_s3(
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
            .bucket("replicapixel-web")
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
            Err(e) => Err(AwsError::RequestFailed(e)), // Wrap the error in Err
        }
    }

    // Get a presigned URL for an object
    pub async fn get_object_pre(&self, s3_key: &S3Key, time: Option<u64>) -> Result<Url, AwsError> {
        self.get_object_pre_base(s3_key, None, time).await
    }

    // Get a presigned URL for an object
    pub async fn get_object_named_pre(
        &self,
        s3_key: &S3Key,
        suggested_filename: String,
        time: Option<u64>,
    ) -> Result<Url, AwsError> {
        self.get_object_pre_base(s3_key, Some(suggested_filename), time)
            .await
    }
    // Get a presigned URL for an object
    pub async fn get_video_pre(&self, video: &VideoModel) -> Result<Url, AwsError> {
        let suggested_filename = Some(Self::sanitize_filename(&video.title));
        let s3_key = S3Key::new(&video.video_s3_key);
        self.get_object_pre_base(&s3_key, suggested_filename, None)
            .await
    }

    // Get a presigned URL for an object
    pub async fn get_thumbnail_pre(&self, video: &VideoModel) -> Result<Url, AwsError> {
        let suggested_filename = Some(video.title.to_string());
        let s3_key = S3Key::new(&video.thumbnail_s3_key);
        self.get_object_pre_base(&s3_key, suggested_filename, None)
            .await
    }

    // Get a presigned URL for an object
    pub async fn get_object_pre_base(
        &self,
        key: &S3Key,
        suggested_filename: Option<String>,
        time: Option<u64>,
    ) -> Result<Url, AwsError> {
        let expires_in = match time {
            Some(t) => t,
            None => self.settings.s3.access_time * 24, // 1 hour * 24 | 1 day
        };
        let suggested_filename = suggested_filename.unwrap_or("image.jpeg".to_string());
        let content_disposition = format!("attachment; filename=\"{}\"", suggested_filename);
        let presigned_request = self
            .client
            .get_object()
            .response_content_disposition(content_disposition)
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

    // Delete an object from a bucket.
    pub async fn remove_video(&self, video: &VideoModel) -> Result<(), AwsError> {
        let s3_key = S3Key::new(&video.video_s3_key);
        self.remove_object_s3_key(&s3_key).await
    }

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

    pub fn init_img_s3_key(user_pid: &Uuid, file_pid: &Uuid) -> S3Key {
        let folder = S3Folders::Images;
        Self::create_s3_key_base(user_pid, file_pid, &folder)
    }
    pub fn init_video_s3_key(user_pid: &Uuid, video_pid: &Uuid) -> S3Key {
        let folder = S3Folders::Video;
        Self::create_s3_key_base(user_pid, video_pid, &folder)
    }
    pub fn init_thumbnail_s3_key(user_pid: &Uuid, video_pid: &Uuid) -> S3Key {
        let folder = S3Folders::Thumbnails;
        Self::create_s3_key_base(user_pid, video_pid, &folder)
    }
    pub fn create_s3_key_base(id: &Uuid, image_name: &Uuid, folder: &S3Folders) -> S3Key {
        let key = format!(
            "{}/{}/{}{}",
            id.to_string(),
            folder.get_folder_str(),
            image_name.to_string(),
            folder.get_file_type(),
        );
        S3Key::new(key)
    }

    fn sanitize_filename(title: &str) -> String {
        let title_str = title
            .trim()
            .to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == ' ' || *c == '-')
            .collect::<String>()
            .replace(' ', "-");
        format!("{}.mp4", title_str)
    }
}

#[async_trait]
impl StoreDriver for AwsS3 {
    async fn upload(&self, path: &Path, content: &Bytes) -> StorageResult<UploadResponse> {
        let path = path.to_string_lossy().to_string();
        let aws = self
            .direct_upload_base(&path, content.clone().into(), "image/jpeg")
            .await
            .map_err(|e| StorageError::StoreNotFound(e.to_string()))?;
        Ok(aws.into())
    }
    async fn get(&self, path: &Path) -> StorageResult<GetResponse> {
        let path = path.to_string_lossy().to_string();
        tracing::warn!(
            "S3 get() not implemented — this driver only supports upload: {}",
            path
        );
        Err(StorageError::Any(
            "get() is not implemented for this storage driver".into(),
        ))
        //     let output = self
        //         .client
        //         .get_object()
        //         .bucket(&self.settings.s3.bucket_name)
        //         .key(&key)
        //         .send()
        //         .await
        //         .map_err(|err| StorageError::StoreNotFound(format!("S3 get_object error: {}", err)))?;

        //     let stream = output.body; // ByteStream from AWS SDK
        //     let reader = stream.into_async_read(); // Convert to AsyncRead

        //     // Convert into `opendal::Reader`-like type using a wrapper if needed
        //     // But since Loco's `GetResponse` expects `opendal::Reader`, you need to adapt or reimplement
        //     // For now, wrap in your custom reader type (simplified version):

        //     let reader = GetResponse::new(reader); // pseudo, adjust if needed
        //     Ok(GetResponse::new(reader))
    }
    async fn delete(&self, path: &Path) -> StorageResult<()> {
        let path = path.to_string_lossy().to_string();
        let removed = self
            .remove_object_s3_key(&S3Key::new(path))
            .await
            .map_err(|e| StorageError::StoreNotFound(e.to_string()));
        removed
    }
    async fn rename(&self, from: &Path, to: &Path) -> StorageResult<()> {
        self.copy(from, to).await?;
        self.delete(from).await?;
        Ok(())
    }
    async fn copy(&self, from: &Path, to: &Path) -> StorageResult<()> {
        let source_key = from.to_string_lossy();
        let destination_key = to.to_string_lossy();

        let copy_source = format!("{}/{}", self.settings.s3.bucket_name, source_key);

        self.client
            .copy_object()
            .bucket(&self.settings.s3.bucket_name)
            .copy_source(copy_source)
            .key(destination_key.to_string())
            .send()
            .await
            .map_err(|e| {
                StorageError::StoreNotFound(format!(
                    "Failed to copy object from '{}' to '{}': {}",
                    source_key, destination_key, e
                ))
            })?;

        Ok(())
    }
    async fn exists(&self, path: &Path) -> StorageResult<bool> {
        let path = path.to_string_lossy().to_string();
        let key = S3Key::new(path);
        let boolean = match self.check_object_exists(&key).await {
            Ok(bool) => bool,
            Err(e) => return Err(StorageError::StoreNotFound(e.to_string())),
        };
        Ok(boolean)
    }
}
