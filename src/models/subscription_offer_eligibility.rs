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
pub enum SubscriptionOfferEligibility {
    #[serde(rename = "STACK_WITH_INTRO_OFFERS")]
    StackWithIntroOffers,
    #[serde(rename = "REPLACE_INTRO_OFFERS")]
    ReplaceIntroOffers,

}

impl std::fmt::Display for SubscriptionOfferEligibility {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::StackWithIntroOffers => write!(f, "STACK_WITH_INTRO_OFFERS"),
            Self::ReplaceIntroOffers => write!(f, "REPLACE_INTRO_OFFERS"),
        }
    }
}

impl Default for SubscriptionOfferEligibility {
    fn default() -> SubscriptionOfferEligibility {
        Self::StackWithIntroOffers
    }
}

