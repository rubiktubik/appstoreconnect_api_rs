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
pub struct AppStoreVersionLocalizationAttributes {
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "locale", skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(rename = "keywords", skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,
    #[serde(rename = "marketingUrl", skip_serializing_if = "Option::is_none")]
    pub marketing_url: Option<String>,
    #[serde(rename = "promotionalText", skip_serializing_if = "Option::is_none")]
    pub promotional_text: Option<String>,
    #[serde(rename = "supportUrl", skip_serializing_if = "Option::is_none")]
    pub support_url: Option<String>,
    #[serde(rename = "whatsNew", skip_serializing_if = "Option::is_none")]
    pub whats_new: Option<String>,
}

impl AppStoreVersionLocalizationAttributes {
    pub fn new() -> AppStoreVersionLocalizationAttributes {
        AppStoreVersionLocalizationAttributes {
            description: None,
            locale: None,
            keywords: None,
            marketing_url: None,
            promotional_text: None,
            support_url: None,
            whats_new: None,
        }
    }
}
