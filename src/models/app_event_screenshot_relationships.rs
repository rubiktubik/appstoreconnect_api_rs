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
pub struct AppEventScreenshotRelationships {
    #[serde(rename = "appEventLocalization", skip_serializing_if = "Option::is_none")]
    pub app_event_localization: Option<Box<models::AppEventScreenshotRelationshipsAppEventLocalization>>,
}

impl AppEventScreenshotRelationships {
    pub fn new() -> AppEventScreenshotRelationships {
        AppEventScreenshotRelationships {
            app_event_localization: None,
        }
    }
}
