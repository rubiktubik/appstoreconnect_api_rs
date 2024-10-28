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
pub struct AppClipRelationships {
    #[serde(rename = "app", skip_serializing_if = "Option::is_none")]
    pub app: Option<Box<models::AlternativeDistributionKeyCreateRequestDataRelationshipsApp>>,
    #[serde(rename = "appClipDefaultExperiences", skip_serializing_if = "Option::is_none")]
    pub app_clip_default_experiences: Option<Box<models::AppClipRelationshipsAppClipDefaultExperiences>>,
    #[serde(rename = "appClipAdvancedExperiences", skip_serializing_if = "Option::is_none")]
    pub app_clip_advanced_experiences: Option<Box<models::AnalyticsReportInstanceRelationshipsSegments>>,
}

impl AppClipRelationships {
    pub fn new() -> AppClipRelationships {
        AppClipRelationships {
            app: None,
            app_clip_default_experiences: None,
            app_clip_advanced_experiences: None,
        }
    }
}
