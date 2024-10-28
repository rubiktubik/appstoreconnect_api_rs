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
pub enum AppStoreVersionExperimentTreatmentsResponseIncludedInner {
    AppStoreVersionExperiment(Box<models::AppStoreVersionExperiment>),
    AppStoreVersionExperimentV2(Box<models::AppStoreVersionExperimentV2>),
    AppStoreVersionExperimentTreatmentLocalization(Box<models::AppStoreVersionExperimentTreatmentLocalization>),
}

impl Default for AppStoreVersionExperimentTreatmentsResponseIncludedInner {
    fn default() -> Self {
        Self::AppStoreVersionExperiment(Default::default())
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "appStoreVersionExperiments")]
    AppStoreVersionExperiments,
    #[serde(rename = "appStoreVersionExperimentTreatmentLocalizations")]
    AppStoreVersionExperimentTreatmentLocalizations,
}

impl Default for Type {
    fn default() -> Type {
        Self::AppStoreVersionExperiments
    }
}
