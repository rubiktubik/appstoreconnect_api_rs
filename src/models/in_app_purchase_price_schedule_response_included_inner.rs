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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InAppPurchasePriceScheduleResponseIncludedInner {
    InAppPurchaseV2(Box<models::InAppPurchaseV2>),
    Territory(Box<models::Territory>),
    InAppPurchasePrice(Box<models::InAppPurchasePrice>),
}

impl Default for InAppPurchasePriceScheduleResponseIncludedInner {
    fn default() -> Self {
        Self::InAppPurchaseV2(Default::default())
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "inAppPurchases")]
    InAppPurchases,
    #[serde(rename = "territories")]
    Territories,
    #[serde(rename = "inAppPurchasePrices")]
    InAppPurchasePrices,
}

impl Default for Type {
    fn default() -> Type {
        Self::InAppPurchases
    }
}

