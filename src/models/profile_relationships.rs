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
pub struct ProfileRelationships {
    #[serde(rename = "bundleId", skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<Box<models::ProfileRelationshipsBundleId>>,
    #[serde(rename = "devices", skip_serializing_if = "Option::is_none")]
    pub devices: Option<Box<models::ProfileRelationshipsDevices>>,
    #[serde(rename = "certificates", skip_serializing_if = "Option::is_none")]
    pub certificates: Option<Box<models::ProfileRelationshipsCertificates>>,
}

impl ProfileRelationships {
    pub fn new() -> ProfileRelationships {
        ProfileRelationships {
            bundle_id: None,
            devices: None,
            certificates: None,
        }
    }
}

