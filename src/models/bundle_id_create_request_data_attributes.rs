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
pub struct BundleIdCreateRequestDataAttributes {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "platform")]
    pub platform: models::BundleIdPlatform,
    #[serde(rename = "identifier")]
    pub identifier: String,
    #[serde(rename = "seedId", skip_serializing_if = "Option::is_none")]
    pub seed_id: Option<String>,
}

impl BundleIdCreateRequestDataAttributes {
    pub fn new(name: String, platform: models::BundleIdPlatform, identifier: String) -> BundleIdCreateRequestDataAttributes {
        BundleIdCreateRequestDataAttributes {
            name,
            platform,
            identifier,
            seed_id: None,
        }
    }
}

