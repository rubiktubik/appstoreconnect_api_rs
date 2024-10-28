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
pub enum SubscriptionOfferMode {
    #[serde(rename = "PAY_AS_YOU_GO")]
    PayAsYouGo,
    #[serde(rename = "PAY_UP_FRONT")]
    PayUpFront,
    #[serde(rename = "FREE_TRIAL")]
    FreeTrial,

}

impl std::fmt::Display for SubscriptionOfferMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::PayAsYouGo => write!(f, "PAY_AS_YOU_GO"),
            Self::PayUpFront => write!(f, "PAY_UP_FRONT"),
            Self::FreeTrial => write!(f, "FREE_TRIAL"),
        }
    }
}

impl Default for SubscriptionOfferMode {
    fn default() -> SubscriptionOfferMode {
        Self::PayAsYouGo
    }
}
