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
pub struct AppEventLocalizationRelationships {
    #[serde(rename = "appEvent", skip_serializing_if = "Option::is_none")]
    pub app_event: Option<Box<models::AppEventLocalizationRelationshipsAppEvent>>,
    #[serde(rename = "appEventScreenshots", skip_serializing_if = "Option::is_none")]
    pub app_event_screenshots: Option<Box<models::AppEventLocalizationRelationshipsAppEventScreenshots>>,
    #[serde(rename = "appEventVideoClips", skip_serializing_if = "Option::is_none")]
    pub app_event_video_clips: Option<Box<models::AppEventLocalizationRelationshipsAppEventVideoClips>>,
}

impl AppEventLocalizationRelationships {
    pub fn new() -> AppEventLocalizationRelationships {
        AppEventLocalizationRelationships {
            app_event: None,
            app_event_screenshots: None,
            app_event_video_clips: None,
        }
    }
}

