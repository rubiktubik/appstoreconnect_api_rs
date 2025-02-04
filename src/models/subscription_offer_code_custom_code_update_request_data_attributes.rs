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
pub struct SubscriptionOfferCodeCustomCodeUpdateRequestDataAttributes {
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
}

impl SubscriptionOfferCodeCustomCodeUpdateRequestDataAttributes {
    pub fn new() -> SubscriptionOfferCodeCustomCodeUpdateRequestDataAttributes {
        SubscriptionOfferCodeCustomCodeUpdateRequestDataAttributes {
            active: None,
        }
    }
}

