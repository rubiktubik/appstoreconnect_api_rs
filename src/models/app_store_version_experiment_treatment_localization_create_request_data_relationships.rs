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
pub struct AppStoreVersionExperimentTreatmentLocalizationCreateRequestDataRelationships {
    #[serde(rename = "appStoreVersionExperimentTreatment")]
    pub app_store_version_experiment_treatment: Box<models::AppStoreVersionExperimentTreatmentLocalizationCreateRequestDataRelationshipsAppStoreVersionExperimentTreatment>,
}

impl AppStoreVersionExperimentTreatmentLocalizationCreateRequestDataRelationships {
    pub fn new(app_store_version_experiment_treatment: models::AppStoreVersionExperimentTreatmentLocalizationCreateRequestDataRelationshipsAppStoreVersionExperimentTreatment) -> AppStoreVersionExperimentTreatmentLocalizationCreateRequestDataRelationships {
        AppStoreVersionExperimentTreatmentLocalizationCreateRequestDataRelationships {
            app_store_version_experiment_treatment: Box::new(app_store_version_experiment_treatment),
        }
    }
}

