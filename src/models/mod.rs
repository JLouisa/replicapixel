pub mod _entities;

pub mod users;
pub type UserActiveModel = users::ActiveModel;
pub type UserModel = users::Model;
pub type UserEntity = users::Entity;

pub mod images;
pub type ImageActiveModel = images::ActiveModel;
pub type ImageModel = images::Model;

pub mod training_models;
pub type TrainingModelActiveModel = training_models::ActiveModel;
pub type TrainingModelModel = training_models::Model;
pub type TrainingModelEntity = training_models::Entity;

pub mod packs;
pub type PackActiveModel = packs::ActiveModel;
pub type PackModel = packs::Model;
pub type PackEntity = packs::Entity;

pub mod transactions;
pub type TransactionActiveModel = transactions::ActiveModel;
pub type TransactionModel = transactions::Model;

pub mod user_credits;
pub type UserCreditActiveModel = user_credits::ActiveModel;
pub type UserCreditModel = user_credits::Model;
pub type UserCreditEntity = user_credits::Entity;

pub mod user_settings;
pub type UserSettingsActiveModel = user_settings::ActiveModel;
pub type UserSettingsModel = user_settings::Model;
pub type UserSettingsEntity = user_settings::Entity;

pub mod plans;
pub type PlanActiveModel = plans::ActiveModel;
pub type PlanModel = plans::Model;
pub type PlanEntity = plans::Entity;

pub mod handled_stripe_events;
pub type StripeEventModel = handled_stripe_events::Model;
pub type StripeEventActiveModel = handled_stripe_events::ActiveModel;
pub type StripeEventsEntity = user_credits::Entity;

pub mod handled_fal_events;
pub type FalEventModel = handled_fal_events::Model;
pub type FalEventActiveModel = handled_fal_events::ActiveModel;
pub type FalEventsEntity = user_credits::Entity;

pub mod notification;
pub type NotificationActiveModel = notification::ActiveModel;
pub type NotificationModel = notification::Model;
pub type NotificationEntity = notification::Entity;

pub mod feature_request;
pub type FeatureRequestActiveModel = feature_request::ActiveModel;
pub type FeatureRequestModel = feature_request::Model;
pub type FeatureRequestEntity = feature_request::Entity;

pub mod feature_vote;
pub type FeatureVoteActiveModel = feature_vote::ActiveModel;
pub type FeatureVoteModel = feature_vote::Model;
pub type FeatureVoteEntity = feature_vote::Entity;

pub mod o_auth2_sessions;
pub type OAuth2SessionActiveModel = o_auth2_sessions::ActiveModel;
pub type OAuth2SessionModel = o_auth2_sessions::Model;
pub type OAuth2SessionEntity = o_auth2_sessions::Entity;

pub mod join;
