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
pub struct AppStoreVersionExperimentTreatmentUpdateRequestDataAttributes {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "appIconName", skip_serializing_if = "Option::is_none")]
    pub app_icon_name: Option<String>,
}

impl AppStoreVersionExperimentTreatmentUpdateRequestDataAttributes {
    pub fn new() -> AppStoreVersionExperimentTreatmentUpdateRequestDataAttributes {
        AppStoreVersionExperimentTreatmentUpdateRequestDataAttributes {
            name: None,
            app_icon_name: None,
        }
    }
}

