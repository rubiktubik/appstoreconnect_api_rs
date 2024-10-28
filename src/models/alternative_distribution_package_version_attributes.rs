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
pub struct AlternativeDistributionPackageVersionAttributes {
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "urlExpirationDate", skip_serializing_if = "Option::is_none")]
    pub url_expiration_date: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "fileChecksum", skip_serializing_if = "Option::is_none")]
    pub file_checksum: Option<String>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
}

impl AlternativeDistributionPackageVersionAttributes {
    pub fn new() -> AlternativeDistributionPackageVersionAttributes {
        AlternativeDistributionPackageVersionAttributes {
            url: None,
            url_expiration_date: None,
            version: None,
            file_checksum: None,
            state: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "COMPLETED")]
    Completed,
    #[serde(rename = "REPLACED")]
    Replaced,
}

impl Default for State {
    fn default() -> State {
        Self::Completed
    }
}

