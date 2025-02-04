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
pub struct CiWorkflowCreateRequestDataAttributes {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description")]
    pub description: String,
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
    #[serde(rename = "actions")]
    pub actions: Vec<models::CiAction>,
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
    #[serde(rename = "isLockedForEditing", skip_serializing_if = "Option::is_none")]
    pub is_locked_for_editing: Option<bool>,
    #[serde(rename = "clean")]
    pub clean: bool,
    #[serde(rename = "containerFilePath")]
    pub container_file_path: String,
}

impl CiWorkflowCreateRequestDataAttributes {
    pub fn new(name: String, description: String, actions: Vec<models::CiAction>, is_enabled: bool, clean: bool, container_file_path: String) -> CiWorkflowCreateRequestDataAttributes {
        CiWorkflowCreateRequestDataAttributes {
            name,
            description,
            branch_start_condition: None,
            tag_start_condition: None,
            pull_request_start_condition: None,
            scheduled_start_condition: None,
            manual_branch_start_condition: None,
            manual_tag_start_condition: None,
            manual_pull_request_start_condition: None,
            actions,
            is_enabled,
            is_locked_for_editing: None,
            clean,
            container_file_path,
        }
    }
}

