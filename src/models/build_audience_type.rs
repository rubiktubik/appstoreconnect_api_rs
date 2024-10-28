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
pub enum BuildAudienceType {
    #[serde(rename = "INTERNAL_ONLY")]
    InternalOnly,
    #[serde(rename = "APP_STORE_ELIGIBLE")]
    AppStoreEligible,

}

impl std::fmt::Display for BuildAudienceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::InternalOnly => write!(f, "INTERNAL_ONLY"),
            Self::AppStoreEligible => write!(f, "APP_STORE_ELIGIBLE"),
        }
    }
}

impl Default for BuildAudienceType {
    fn default() -> BuildAudienceType {
        Self::InternalOnly
    }
}

