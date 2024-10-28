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
pub struct AppClipDefaultExperienceRelationships {
    #[serde(rename = "appClip", skip_serializing_if = "Option::is_none")]
    pub app_clip: Option<Box<models::AppClipAdvancedExperienceRelationshipsAppClip>>,
    #[serde(rename = "releaseWithAppStoreVersion", skip_serializing_if = "Option::is_none")]
    pub release_with_app_store_version: Option<Box<models::AppClipDefaultExperienceRelationshipsReleaseWithAppStoreVersion>>,
    #[serde(rename = "appClipDefaultExperienceLocalizations", skip_serializing_if = "Option::is_none")]
    pub app_clip_default_experience_localizations: Option<Box<models::AppClipDefaultExperienceRelationshipsAppClipDefaultExperienceLocalizations>>,
    #[serde(rename = "appClipAppStoreReviewDetail", skip_serializing_if = "Option::is_none")]
    pub app_clip_app_store_review_detail: Option<Box<models::AppClipDefaultExperienceRelationshipsAppClipAppStoreReviewDetail>>,
}

impl AppClipDefaultExperienceRelationships {
    pub fn new() -> AppClipDefaultExperienceRelationships {
        AppClipDefaultExperienceRelationships {
            app_clip: None,
            release_with_app_store_version: None,
            app_clip_default_experience_localizations: None,
            app_clip_app_store_review_detail: None,
        }
    }
}

