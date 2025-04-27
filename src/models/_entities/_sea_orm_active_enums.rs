use derive_more::Display;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use strum::{EnumIter, EnumString};

#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    PartialEq,
    Eq,
    EnumIter,
    DeriveActiveEnum,
    Serialize,
    Deserialize,
    PartialOrd,
    Ord,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "sex")]
pub enum Sex {
    #[sea_orm(string_value = "Male")]
    Male,
    #[sea_orm(string_value = "Female")]
    Female,
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    EnumIter,
    DeriveActiveEnum,
    EnumString,
    Display,
    Serialize,
    Deserialize,
    PartialOrd,
    Ord,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "plan_names")]
#[serde(rename_all = "lowercase")]
pub enum PlanNames {
    #[sea_orm(string_value = "Basic")]
    Basic,
    #[sea_orm(string_value = "Premium")]
    Premium,
    #[sea_orm(string_value = "Max")]
    Max,
}

#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    PartialEq,
    Eq,
    EnumIter,
    DeriveActiveEnum,
    Serialize,
    Deserialize,
    PartialOrd,
    Ord,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "status")]
pub enum Status {
    #[sea_orm(string_value = "Completed")]
    Completed,
    #[sea_orm(string_value = "Training")]
    Training,
    #[sea_orm(string_value = "Pending")]
    Pending,
    #[sea_orm(string_value = "Processing")]
    Processing,
    #[sea_orm(string_value = "Failed")]
    Failed,
    #[sea_orm(string_value = "Cancelled")]
    Cancelled,
}
impl Default for Status {
    fn default() -> Self {
        Status::Pending
    }
}

#[derive(
    Serialize, Deserialize, Debug, Clone, Copy, EnumIter, Display, DeriveActiveEnum, PartialEq, Eq,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "image_format")]
#[serde(rename_all = "lowercase")]
pub enum ImageFormat {
    #[sea_orm(string_value = "png")]
    #[strum(to_string = "png")]
    Png,
    #[sea_orm(string_value = "jpg")]
    #[strum(to_string = "jpg")]
    Jpeg,
    #[sea_orm(string_value = "zip")]
    #[strum(to_string = "zip")]
    Zip,
}

#[derive(
    Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize, PartialOrd, Ord,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "language")]
pub enum Language {
    #[sea_orm(string_value = "English")]
    #[strum(to_string = "en-US")]
    #[serde(rename = "en-US")]
    English,
    #[sea_orm(string_value = "Spanish")]
    #[strum(to_string = "es-ES")]
    #[serde(rename = "es-ES")]
    Spanish,
    #[sea_orm(string_value = "German")]
    #[strum(to_string = "de-DE")]
    #[serde(rename = "de-DE")]
    German,
    #[sea_orm(string_value = "Italian")]
    #[strum(to_string = "it-IT")]
    #[serde(rename = "it-IT")]
    Italian,
    #[sea_orm(string_value = "Dutch")]
    #[strum(to_string = "nl-NL")]
    #[serde(rename = "nl-NL")]
    Dutch,
}
impl Default for Language {
    fn default() -> Self {
        Language::English
    }
}

#[derive(
    Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize, PartialOrd, Ord,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "notification")]
pub enum Notification {
    #[sea_orm(string_value = "Message")]
    Message,
    #[sea_orm(string_value = "System_update")]
    SystemUpdate,
    #[sea_orm(string_value = "Promotion")]
    Promotion,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    EnumIter,
    Serialize,
    Deserialize,
    strum_macros::Display,
    DeriveActiveEnum,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "eye_color")]
pub enum EyeColor {
    #[sea_orm(string_value = "Brown")]
    Brown,
    #[sea_orm(string_value = "Blue")]
    Blue,
    #[sea_orm(string_value = "Green")]
    Green,
    #[sea_orm(string_value = "Grey")]
    Grey,
    #[sea_orm(string_value = "Hazel")]
    Hazel,
    #[sea_orm(string_value = "Red")]
    Red,
}

#[derive(
    Clone,
    Copy,
    Debug,
    Serialize,
    Deserialize,
    EnumIter,
    strum_macros::Display,
    DeriveActiveEnum,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "ethnicity")]
pub enum Ethnicity {
    #[sea_orm(string_value = "White")]
    White,
    #[sea_orm(string_value = "Black")]
    Black,
    #[sea_orm(string_value = "Pacific")]
    Pacific,
    #[sea_orm(string_value = "Hispanic")]
    Hispanic,
    #[sea_orm(string_value = "Asian")]
    Asian,
    #[sea_orm(string_value = "SouthEastAsian")]
    #[serde(rename = "South East Asian")]
    #[strum(to_string = "South East Asian")]
    SouthEastAsian,
    #[sea_orm(string_value = "SouthAsian")]
    #[serde(rename = "South Asian")]
    #[strum(to_string = "South Asian")]
    SouthAsian,
    #[sea_orm(string_value = "MiddleEastern")]
    #[serde(rename = "Middle Eastern")]
    #[strum(to_string = "Middle Eastern")]
    MiddleEastern,
}

#[derive(
    Clone,
    Copy,
    Debug,
    Serialize,
    Deserialize,
    EnumIter,
    strum_macros::Display,
    DeriveActiveEnum,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "emotion")]
pub enum Emotion {
    #[sea_orm(string_value = "Neutral")]
    Neutral,
    #[sea_orm(string_value = "Anger")]
    Anger,
    #[sea_orm(string_value = "Disgust")]
    Disgust,
    #[sea_orm(string_value = "Fear")]
    Fear,
    #[sea_orm(string_value = "Happy")]
    Happy,
    #[sea_orm(string_value = "Sad")]
    Sad,
    #[sea_orm(string_value = "Surprise")]
    Surprise,
}

#[derive(
    Clone,
    Copy,
    Debug,
    Serialize,
    Deserialize,
    EnumIter,
    strum_macros::Display,
    DeriveActiveEnum,
    PartialEq,
    Eq,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "based_on")]
pub enum BasedOn {
    #[sea_orm(string_value = "RealPerson")]
    #[serde(rename = "Based on a real person")]
    #[strum(to_string = "Based on a real person")]
    RealPerson,
    #[sea_orm(string_value = "CreateInfluencerAI")]
    #[serde(rename = "Create a new AI influencer")]
    #[strum(to_string = "Create a new AI influencer")]
    CreateInfluencerAI,
}

#[derive(
    Clone,
    Copy,
    Debug,
    Serialize,
    Deserialize,
    EnumString,
    EnumIter,
    PartialEq,
    Eq,
    strum_macros::Display,
    DeriveActiveEnum,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "image_size")]
pub enum ImageSize {
    #[strum(to_string = "512x512 (Square)")]
    #[sea_orm(string_value = "Square")]
    #[serde(rename = "square")]
    Square,
    #[strum(to_string = "1024x1024 (Square HD)")]
    #[sea_orm(string_value = "SquareHD")]
    #[serde(rename = "square_hd")]
    SquareHD,
    #[strum(to_string = "768x1024 (Portrait 4:3)")]
    #[sea_orm(string_value = "Portrait43")]
    #[serde(rename = "portrait_4_3")]
    Portrait43,
    #[strum(to_string = "576x1024 (Portrait 16:9)")]
    #[sea_orm(string_value = "Portrait169")]
    #[serde(rename = "portrait_16_9")]
    Portrait169,
    #[strum(to_string = "1024x768 (Landscape 4:3)")]
    #[sea_orm(string_value = "Landscape43")]
    #[serde(rename = "landscape_4_3")]
    Landscape43,
    #[strum(to_string = "1024x576 (Landscape 16:9)")]
    #[sea_orm(string_value = "Landscape169")]
    #[serde(rename = "landscape_16_9")]
    Landscape169,
}
impl Default for ImageSize {
    fn default() -> Self {
        Self::Square
    }
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "theme_preference")]
#[serde(rename_all = "lowercase")]
pub enum ThemePreference {
    #[sea_orm(string_value = "Light")]
    Light,
    #[sea_orm(string_value = "Dark")]
    Dark,
    #[sea_orm(string_value = "System")]
    System,
}
impl Default for ThemePreference {
    fn default() -> Self {
        ThemePreference::Dark
    }
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "feature_status")]
pub enum FeatureStatus {
    #[sea_orm(string_value = "Suggested")]
    Suggested,
    #[sea_orm(string_value = "Planned")]
    Planned,
    #[sea_orm(string_value = "In_progress")]
    InProgress,
    #[sea_orm(string_value = "Completed")]
    Completed,
    #[sea_orm(string_value = "Rejected")]
    Rejected,
}

#[derive(Serialize, Debug, Clone, EnumString)]
pub enum CheckOutStatus {
    Succeeded,
    Cancelled,
    Processing,
}
