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
pub struct AppStoreVersionExperimentTreatmentAttributes {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "appIcon", skip_serializing_if = "Option::is_none")]
    pub app_icon: Option<Box<models::ImageAsset>>,
    #[serde(rename = "appIconName", skip_serializing_if = "Option::is_none")]
    pub app_icon_name: Option<String>,
    #[serde(rename = "promotedDate", skip_serializing_if = "Option::is_none")]
    pub promoted_date: Option<String>,
}

impl AppStoreVersionExperimentTreatmentAttributes {
    pub fn new() -> AppStoreVersionExperimentTreatmentAttributes {
        AppStoreVersionExperimentTreatmentAttributes {
            name: None,
            app_icon: None,
            app_icon_name: None,
            promoted_date: None,
        }
    }
}
