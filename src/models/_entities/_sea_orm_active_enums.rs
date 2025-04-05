use derive_more::Display;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use strum::{EnumIter, EnumString, IntoEnumIterator};

#[derive(
    Debug, Clone, Copy, Display, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "sex")]
pub enum Sex {
    #[sea_orm(string_value = "Male")]
    Male,
    #[sea_orm(string_value = "Female")]
    Female,
}

#[derive(
    Debug, Clone, Copy, Display, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "status")]
pub enum Status {
    #[sea_orm(string_value = "Pending")]
    Pending,
    #[sea_orm(string_value = "Processing")]
    Processing,
    #[sea_orm(string_value = "Training")]
    Training,
    #[sea_orm(string_value = "Completed")]
    Completed,
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
    #[sea_orm(string_value = "Png")]
    #[strum(to_string = "png")]
    Png,
    #[sea_orm(string_value = "Jpeg")]
    #[strum(to_string = "jpeg")]
    Jpeg,
    #[sea_orm(string_value = "Zip")]
    #[strum(to_string = "zip")]
    Zip,
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
