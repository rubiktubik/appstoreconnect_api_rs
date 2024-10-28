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
pub struct AppPreviewRelationships {
    #[serde(rename = "appPreviewSet", skip_serializing_if = "Option::is_none")]
    pub app_preview_set: Option<Box<models::AppPreviewRelationshipsAppPreviewSet>>,
}

impl AppPreviewRelationships {
    pub fn new() -> AppPreviewRelationships {
        AppPreviewRelationships {
            app_preview_set: None,
        }
    }
}

