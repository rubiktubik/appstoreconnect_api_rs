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
pub struct BundleIdAttributes {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "platform", skip_serializing_if = "Option::is_none")]
    pub platform: Option<models::BundleIdPlatform>,
    #[serde(rename = "identifier", skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(rename = "seedId", skip_serializing_if = "Option::is_none")]
    pub seed_id: Option<String>,
}

impl BundleIdAttributes {
    pub fn new() -> BundleIdAttributes {
        BundleIdAttributes {
            name: None,
            platform: None,
            identifier: None,
            seed_id: None,
        }
    }
}

