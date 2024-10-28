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
pub struct AppClipAdvancedExperienceLocalizationAttributes {
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<models::AppClipAdvancedExperienceLanguage>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "subtitle", skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
}

impl AppClipAdvancedExperienceLocalizationAttributes {
    pub fn new() -> AppClipAdvancedExperienceLocalizationAttributes {
        AppClipAdvancedExperienceLocalizationAttributes {
            language: None,
            title: None,
            subtitle: None,
        }
    }
}
