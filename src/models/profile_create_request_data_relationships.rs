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
pub struct ProfileCreateRequestDataRelationships {
    #[serde(rename = "bundleId")]
    pub bundle_id: Box<models::BundleIdCapabilityCreateRequestDataRelationshipsBundleId>,
    #[serde(rename = "devices", skip_serializing_if = "Option::is_none")]
    pub devices: Option<Box<models::ProfileCreateRequestDataRelationshipsDevices>>,
    #[serde(rename = "certificates")]
    pub certificates: Box<models::ProfileCreateRequestDataRelationshipsCertificates>,
}

impl ProfileCreateRequestDataRelationships {
    pub fn new(bundle_id: models::BundleIdCapabilityCreateRequestDataRelationshipsBundleId, certificates: models::ProfileCreateRequestDataRelationshipsCertificates) -> ProfileCreateRequestDataRelationships {
        ProfileCreateRequestDataRelationships {
            bundle_id: Box::new(bundle_id),
            devices: None,
            certificates: Box::new(certificates),
        }
    }
}

