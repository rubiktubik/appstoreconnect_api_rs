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
pub struct CiWorkflowAttributes {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "branchStartCondition", skip_serializing_if = "Option::is_none")]
    pub branch_start_condition: Option<Box<models::CiBranchStartCondition>>,
    #[serde(rename = "tagStartCondition", skip_serializing_if = "Option::is_none")]
    pub tag_start_condition: Option<Box<models::CiTagStartCondition>>,
    #[serde(rename = "pullRequestStartCondition", skip_serializing_if = "Option::is_none")]
    pub pull_request_start_condition: Option<Box<models::CiPullRequestStartCondition>>,
    #[serde(rename = "scheduledStartCondition", skip_serializing_if = "Option::is_none")]
    pub scheduled_start_condition: Option<Box<models::CiScheduledStartCondition>>,
    #[serde(rename = "manualBranchStartCondition", skip_serializing_if = "Option::is_none")]
    pub manual_branch_start_condition: Option<Box<models::CiManualBranchStartCondition>>,
    #[serde(rename = "manualTagStartCondition", skip_serializing_if = "Option::is_none")]
    pub manual_tag_start_condition: Option<Box<models::CiManualTagStartCondition>>,
    #[serde(rename = "manualPullRequestStartCondition", skip_serializing_if = "Option::is_none")]
    pub manual_pull_request_start_condition: Option<Box<models::CiManualPullRequestStartCondition>>,
    #[serde(rename = "actions", skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<models::CiAction>>,
    #[serde(rename = "isEnabled", skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[serde(rename = "isLockedForEditing", skip_serializing_if = "Option::is_none")]
    pub is_locked_for_editing: Option<bool>,
    #[serde(rename = "clean", skip_serializing_if = "Option::is_none")]
    pub clean: Option<bool>,
    #[serde(rename = "containerFilePath", skip_serializing_if = "Option::is_none")]
    pub container_file_path: Option<String>,
    #[serde(rename = "lastModifiedDate", skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
}

impl CiWorkflowAttributes {
    pub fn new() -> CiWorkflowAttributes {
        CiWorkflowAttributes {
            name: None,
            description: None,
            branch_start_condition: None,
            tag_start_condition: None,
            pull_request_start_condition: None,
            scheduled_start_condition: None,
            manual_branch_start_condition: None,
            manual_tag_start_condition: None,
            manual_pull_request_start_condition: None,
            actions: None,
            is_enabled: None,
            is_locked_for_editing: None,
            clean: None,
            container_file_path: None,
            last_modified_date: None,
        }
    }
}

