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
pub struct SubscriptionPriceAttributes {
    #[serde(rename = "startDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(rename = "preserved", skip_serializing_if = "Option::is_none")]
    pub preserved: Option<bool>,
}

impl SubscriptionPriceAttributes {
    pub fn new() -> SubscriptionPriceAttributes {
        SubscriptionPriceAttributes {
            start_date: None,
            preserved: None,
        }
    }
}
