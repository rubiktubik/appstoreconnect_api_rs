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
pub struct AlternativeDistributionPackageCreateRequestDataRelationships {
    #[serde(rename = "appStoreVersion")]
    pub app_store_version: Box<models::AlternativeDistributionPackageCreateRequestDataRelationshipsAppStoreVersion>,
}

impl AlternativeDistributionPackageCreateRequestDataRelationships {
    pub fn new(app_store_version: models::AlternativeDistributionPackageCreateRequestDataRelationshipsAppStoreVersion) -> AlternativeDistributionPackageCreateRequestDataRelationships {
        AlternativeDistributionPackageCreateRequestDataRelationships {
            app_store_version: Box::new(app_store_version),
        }
    }
}
