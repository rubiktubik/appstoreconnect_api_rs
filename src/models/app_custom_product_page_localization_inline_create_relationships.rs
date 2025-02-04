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
pub struct AppCustomProductPageLocalizationInlineCreateRelationships {
    #[serde(rename = "appCustomProductPageVersion", skip_serializing_if = "Option::is_none")]
    pub app_custom_product_page_version: Option<Box<models::AppCustomProductPageLocalizationRelationshipsAppCustomProductPageVersion>>,
}

impl AppCustomProductPageLocalizationInlineCreateRelationships {
    pub fn new() -> AppCustomProductPageLocalizationInlineCreateRelationships {
        AppCustomProductPageLocalizationInlineCreateRelationships {
            app_custom_product_page_version: None,
        }
    }
}

