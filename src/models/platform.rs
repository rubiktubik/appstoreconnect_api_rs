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
pub enum Platform {
    #[serde(rename = "IOS")]
    Ios,
    #[serde(rename = "MAC_OS")]
    MacOs,
    #[serde(rename = "TV_OS")]
    TvOs,
    #[serde(rename = "VISION_OS")]
    VisionOs,

}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Ios => write!(f, "IOS"),
            Self::MacOs => write!(f, "MAC_OS"),
            Self::TvOs => write!(f, "TV_OS"),
            Self::VisionOs => write!(f, "VISION_OS"),
        }
    }
}

impl Default for Platform {
    fn default() -> Platform {
        Self::Ios
    }
}

