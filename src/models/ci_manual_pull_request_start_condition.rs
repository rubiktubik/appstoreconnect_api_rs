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
pub struct CiManualPullRequestStartCondition {
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<Box<models::CiBranchPatterns>>,
    #[serde(rename = "destination", skip_serializing_if = "Option::is_none")]
    pub destination: Option<Box<models::CiBranchPatterns>>,
}

impl CiManualPullRequestStartCondition {
    pub fn new() -> CiManualPullRequestStartCondition {
        CiManualPullRequestStartCondition {
            source: None,
            destination: None,
        }
    }
}

