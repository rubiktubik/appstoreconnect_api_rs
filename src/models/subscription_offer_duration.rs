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
pub enum SubscriptionOfferDuration {
    #[serde(rename = "THREE_DAYS")]
    ThreeDays,
    #[serde(rename = "ONE_WEEK")]
    OneWeek,
    #[serde(rename = "TWO_WEEKS")]
    TwoWeeks,
    #[serde(rename = "ONE_MONTH")]
    OneMonth,
    #[serde(rename = "TWO_MONTHS")]
    TwoMonths,
    #[serde(rename = "THREE_MONTHS")]
    ThreeMonths,
    #[serde(rename = "SIX_MONTHS")]
    SixMonths,
    #[serde(rename = "ONE_YEAR")]
    OneYear,

}

impl std::fmt::Display for SubscriptionOfferDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ThreeDays => write!(f, "THREE_DAYS"),
            Self::OneWeek => write!(f, "ONE_WEEK"),
            Self::TwoWeeks => write!(f, "TWO_WEEKS"),
            Self::OneMonth => write!(f, "ONE_MONTH"),
            Self::TwoMonths => write!(f, "TWO_MONTHS"),
            Self::ThreeMonths => write!(f, "THREE_MONTHS"),
            Self::SixMonths => write!(f, "SIX_MONTHS"),
            Self::OneYear => write!(f, "ONE_YEAR"),
        }
    }
}

impl Default for SubscriptionOfferDuration {
    fn default() -> SubscriptionOfferDuration {
        Self::ThreeDays
    }
}

