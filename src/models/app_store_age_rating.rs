/*
 * App Store Connect API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.6.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AppStoreAgeRating {
    #[serde(rename = "FOUR_PLUS")]
    FourPlus,
    #[serde(rename = "NINE_PLUS")]
    NinePlus,
    #[serde(rename = "TWELVE_PLUS")]
    TwelvePlus,
    #[serde(rename = "SEVENTEEN_PLUS")]
    SeventeenPlus,
    #[serde(rename = "UNRATED")]
    Unrated,

}

impl std::fmt::Display for AppStoreAgeRating {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FourPlus => write!(f, "FOUR_PLUS"),
            Self::NinePlus => write!(f, "NINE_PLUS"),
            Self::TwelvePlus => write!(f, "TWELVE_PLUS"),
            Self::SeventeenPlus => write!(f, "SEVENTEEN_PLUS"),
            Self::Unrated => write!(f, "UNRATED"),
        }
    }
}

impl Default for AppStoreAgeRating {
    fn default() -> AppStoreAgeRating {
        Self::FourPlus
    }
}

