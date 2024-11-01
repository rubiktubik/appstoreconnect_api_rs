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
pub enum SubscriptionCustomerEligibility {
    #[serde(rename = "NEW")]
    New,
    #[serde(rename = "EXISTING")]
    Existing,
    #[serde(rename = "EXPIRED")]
    Expired,

}

impl std::fmt::Display for SubscriptionCustomerEligibility {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::New => write!(f, "NEW"),
            Self::Existing => write!(f, "EXISTING"),
            Self::Expired => write!(f, "EXPIRED"),
        }
    }
}

impl Default for SubscriptionCustomerEligibility {
    fn default() -> SubscriptionCustomerEligibility {
        Self::New
    }
}

