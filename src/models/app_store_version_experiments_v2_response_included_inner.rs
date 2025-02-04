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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AppStoreVersionExperimentsV2ResponseIncludedInner {
    App(Box<models::App>),
    AppStoreVersion(Box<models::AppStoreVersion>),
    AppStoreVersionExperimentTreatment(Box<models::AppStoreVersionExperimentTreatment>),
}

impl Default for AppStoreVersionExperimentsV2ResponseIncludedInner {
    fn default() -> Self {
        Self::App(Default::default())
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "apps")]
    Apps,
    #[serde(rename = "appStoreVersions")]
    AppStoreVersions,
    #[serde(rename = "appStoreVersionExperimentTreatments")]
    AppStoreVersionExperimentTreatments,
}

impl Default for Type {
    fn default() -> Type {
        Self::Apps
    }
}

