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
pub struct BetaBuildLocalizationAttributes {
    #[serde(rename = "whatsNew", skip_serializing_if = "Option::is_none")]
    pub whats_new: Option<String>,
    #[serde(rename = "locale", skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
}

impl BetaBuildLocalizationAttributes {
    pub fn new() -> BetaBuildLocalizationAttributes {
        BetaBuildLocalizationAttributes {
            whats_new: None,
            locale: None,
        }
    }
}

