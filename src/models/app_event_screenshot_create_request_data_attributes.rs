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
pub struct AppEventScreenshotCreateRequestDataAttributes {
    #[serde(rename = "fileSize")]
    pub file_size: i32,
    #[serde(rename = "fileName")]
    pub file_name: String,
    #[serde(rename = "appEventAssetType")]
    pub app_event_asset_type: models::AppEventAssetType,
}

impl AppEventScreenshotCreateRequestDataAttributes {
    pub fn new(file_size: i32, file_name: String, app_event_asset_type: models::AppEventAssetType) -> AppEventScreenshotCreateRequestDataAttributes {
        AppEventScreenshotCreateRequestDataAttributes {
            file_size,
            file_name,
            app_event_asset_type,
        }
    }
}

