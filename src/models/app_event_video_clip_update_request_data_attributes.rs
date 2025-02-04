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
pub struct AppEventVideoClipUpdateRequestDataAttributes {
    #[serde(rename = "previewFrameTimeCode", skip_serializing_if = "Option::is_none")]
    pub preview_frame_time_code: Option<String>,
    #[serde(rename = "uploaded", skip_serializing_if = "Option::is_none")]
    pub uploaded: Option<bool>,
}

impl AppEventVideoClipUpdateRequestDataAttributes {
    pub fn new() -> AppEventVideoClipUpdateRequestDataAttributes {
        AppEventVideoClipUpdateRequestDataAttributes {
            preview_frame_time_code: None,
            uploaded: None,
        }
    }
}

