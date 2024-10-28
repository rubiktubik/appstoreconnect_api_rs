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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubscriptionPromotionalOfferPriceInlineCreate {
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "relationships", skip_serializing_if = "Option::is_none")]
    pub relationships: Option<Box<models::SubscriptionOfferCodePriceRelationships>>,
}

impl SubscriptionPromotionalOfferPriceInlineCreate {
    pub fn new(r#type: Type) -> SubscriptionPromotionalOfferPriceInlineCreate {
        SubscriptionPromotionalOfferPriceInlineCreate {
            r#type,
            id: None,
            relationships: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "subscriptionPromotionalOfferPrices")]
    SubscriptionPromotionalOfferPrices,
}

impl Default for Type {
    fn default() -> Type {
        Self::SubscriptionPromotionalOfferPrices
    }
}

