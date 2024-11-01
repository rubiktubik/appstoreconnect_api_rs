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
pub struct AppScreenshotSetRelationships {
    #[serde(rename = "appStoreVersionLocalization", skip_serializing_if = "Option::is_none")]
    pub app_store_version_localization: Option<Box<models::AppPreviewSetRelationshipsAppStoreVersionLocalization>>,
    #[serde(rename = "appCustomProductPageLocalization", skip_serializing_if = "Option::is_none")]
    pub app_custom_product_page_localization: Option<Box<models::AppPreviewSetRelationshipsAppCustomProductPageLocalization>>,
    #[serde(rename = "appStoreVersionExperimentTreatmentLocalization", skip_serializing_if = "Option::is_none")]
    pub app_store_version_experiment_treatment_localization: Option<Box<models::AppPreviewSetRelationshipsAppStoreVersionExperimentTreatmentLocalization>>,
    #[serde(rename = "appScreenshots", skip_serializing_if = "Option::is_none")]
    pub app_screenshots: Option<Box<models::AppScreenshotSetRelationshipsAppScreenshots>>,
}

impl AppScreenshotSetRelationships {
    pub fn new() -> AppScreenshotSetRelationships {
        AppScreenshotSetRelationships {
            app_store_version_localization: None,
            app_custom_product_page_localization: None,
            app_store_version_experiment_treatment_localization: None,
            app_screenshots: None,
        }
    }
}

