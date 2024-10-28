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
pub struct AppPricePointV3Attributes {
    #[serde(rename = "customerPrice", skip_serializing_if = "Option::is_none")]
    pub customer_price: Option<String>,
    #[serde(rename = "proceeds", skip_serializing_if = "Option::is_none")]
    pub proceeds: Option<String>,
}

impl AppPricePointV3Attributes {
    pub fn new() -> AppPricePointV3Attributes {
        AppPricePointV3Attributes {
            customer_price: None,
            proceeds: None,
        }
    }
}

