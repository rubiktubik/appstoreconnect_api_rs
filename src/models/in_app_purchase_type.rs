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
pub enum InAppPurchaseType {
    #[serde(rename = "CONSUMABLE")]
    Consumable,
    #[serde(rename = "NON_CONSUMABLE")]
    NonConsumable,
    #[serde(rename = "NON_RENEWING_SUBSCRIPTION")]
    NonRenewingSubscription,

}

impl std::fmt::Display for InAppPurchaseType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Consumable => write!(f, "CONSUMABLE"),
            Self::NonConsumable => write!(f, "NON_CONSUMABLE"),
            Self::NonRenewingSubscription => write!(f, "NON_RENEWING_SUBSCRIPTION"),
        }
    }
}

impl Default for InAppPurchaseType {
    fn default() -> InAppPurchaseType {
        Self::Consumable
    }
}
