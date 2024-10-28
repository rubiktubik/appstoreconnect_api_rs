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
pub struct CiPullRequestStartCondition {
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<Box<models::CiBranchPatterns>>,
    #[serde(rename = "destination", skip_serializing_if = "Option::is_none")]
    pub destination: Option<Box<models::CiBranchPatterns>>,
    #[serde(rename = "filesAndFoldersRule", skip_serializing_if = "Option::is_none")]
    pub files_and_folders_rule: Option<Box<models::CiFilesAndFoldersRule>>,
    #[serde(rename = "autoCancel", skip_serializing_if = "Option::is_none")]
    pub auto_cancel: Option<bool>,
}

impl CiPullRequestStartCondition {
    pub fn new() -> CiPullRequestStartCondition {
        CiPullRequestStartCondition {
            source: None,
            destination: None,
            files_and_folders_rule: None,
            auto_cancel: None,
        }
    }
}

