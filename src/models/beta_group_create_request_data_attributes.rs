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
pub struct BetaGroupCreateRequestDataAttributes {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "isInternalGroup", skip_serializing_if = "Option::is_none")]
    pub is_internal_group: Option<bool>,
    #[serde(rename = "hasAccessToAllBuilds", skip_serializing_if = "Option::is_none")]
    pub has_access_to_all_builds: Option<bool>,
    #[serde(rename = "publicLinkEnabled", skip_serializing_if = "Option::is_none")]
    pub public_link_enabled: Option<bool>,
    #[serde(rename = "publicLinkLimitEnabled", skip_serializing_if = "Option::is_none")]
    pub public_link_limit_enabled: Option<bool>,
    #[serde(rename = "publicLinkLimit", skip_serializing_if = "Option::is_none")]
    pub public_link_limit: Option<i32>,
    #[serde(rename = "feedbackEnabled", skip_serializing_if = "Option::is_none")]
    pub feedback_enabled: Option<bool>,
}

impl BetaGroupCreateRequestDataAttributes {
    pub fn new(name: String) -> BetaGroupCreateRequestDataAttributes {
        BetaGroupCreateRequestDataAttributes {
            name,
            is_internal_group: None,
            has_access_to_all_builds: None,
            public_link_enabled: None,
            public_link_limit_enabled: None,
            public_link_limit: None,
            feedback_enabled: None,
        }
    }
}

