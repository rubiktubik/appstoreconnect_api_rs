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
pub struct AppScreenshotSetAttributes {
    #[serde(rename = "screenshotDisplayType", skip_serializing_if = "Option::is_none")]
    pub screenshot_display_type: Option<models::ScreenshotDisplayType>,
}

impl AppScreenshotSetAttributes {
    pub fn new() -> AppScreenshotSetAttributes {
        AppScreenshotSetAttributes {
            screenshot_display_type: None,
        }
    }
}

