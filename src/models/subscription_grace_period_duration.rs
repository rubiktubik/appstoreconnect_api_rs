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
pub enum SubscriptionGracePeriodDuration {
    #[serde(rename = "THREE_DAYS")]
    ThreeDays,
    #[serde(rename = "SIXTEEN_DAYS")]
    SixteenDays,
    #[serde(rename = "TWENTY_EIGHT_DAYS")]
    TwentyEightDays,

}

impl std::fmt::Display for SubscriptionGracePeriodDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ThreeDays => write!(f, "THREE_DAYS"),
            Self::SixteenDays => write!(f, "SIXTEEN_DAYS"),
            Self::TwentyEightDays => write!(f, "TWENTY_EIGHT_DAYS"),
        }
    }
}

impl Default for SubscriptionGracePeriodDuration {
    fn default() -> SubscriptionGracePeriodDuration {
        Self::ThreeDays
    }
}

