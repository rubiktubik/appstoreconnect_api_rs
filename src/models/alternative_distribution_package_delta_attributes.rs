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
pub struct AlternativeDistributionPackageDeltaAttributes {
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "urlExpirationDate", skip_serializing_if = "Option::is_none")]
    pub url_expiration_date: Option<String>,
    #[serde(rename = "alternativeDistributionKeyBlob", skip_serializing_if = "Option::is_none")]
    pub alternative_distribution_key_blob: Option<String>,
    #[serde(rename = "fileChecksum", skip_serializing_if = "Option::is_none")]
    pub file_checksum: Option<String>,
}

impl AlternativeDistributionPackageDeltaAttributes {
    pub fn new() -> AlternativeDistributionPackageDeltaAttributes {
        AlternativeDistributionPackageDeltaAttributes {
            url: None,
            url_expiration_date: None,
            alternative_distribution_key_blob: None,
            file_checksum: None,
        }
    }
}
