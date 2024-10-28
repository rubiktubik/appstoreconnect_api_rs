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
pub struct SubscriptionOfferCodeOneTimeUseCodeAttributes {
    #[serde(rename = "numberOfCodes", skip_serializing_if = "Option::is_none")]
    pub number_of_codes: Option<i32>,
    #[serde(rename = "createdDate", skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    #[serde(rename = "expirationDate", skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
}

impl SubscriptionOfferCodeOneTimeUseCodeAttributes {
    pub fn new() -> SubscriptionOfferCodeOneTimeUseCodeAttributes {
        SubscriptionOfferCodeOneTimeUseCodeAttributes {
            number_of_codes: None,
            created_date: None,
            expiration_date: None,
            active: None,
        }
    }
}

