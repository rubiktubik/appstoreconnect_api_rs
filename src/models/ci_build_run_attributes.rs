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
pub struct CiBuildRunAttributes {
    #[serde(rename = "number", skip_serializing_if = "Option::is_none")]
    pub number: Option<i32>,
    #[serde(rename = "createdDate", skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    #[serde(rename = "startedDate", skip_serializing_if = "Option::is_none")]
    pub started_date: Option<String>,
    #[serde(rename = "finishedDate", skip_serializing_if = "Option::is_none")]
    pub finished_date: Option<String>,
    #[serde(rename = "sourceCommit", skip_serializing_if = "Option::is_none")]
    pub source_commit: Option<Box<models::CiBuildRunAttributesSourceCommit>>,
    #[serde(rename = "destinationCommit", skip_serializing_if = "Option::is_none")]
    pub destination_commit: Option<Box<models::CiBuildRunAttributesSourceCommit>>,
    #[serde(rename = "isPullRequestBuild", skip_serializing_if = "Option::is_none")]
    pub is_pull_request_build: Option<bool>,
    #[serde(rename = "issueCounts", skip_serializing_if = "Option::is_none")]
    pub issue_counts: Option<Box<models::CiIssueCounts>>,
    #[serde(rename = "executionProgress", skip_serializing_if = "Option::is_none")]
    pub execution_progress: Option<models::CiExecutionProgress>,
    #[serde(rename = "completionStatus", skip_serializing_if = "Option::is_none")]
    pub completion_status: Option<models::CiCompletionStatus>,
    #[serde(rename = "startReason", skip_serializing_if = "Option::is_none")]
    pub start_reason: Option<StartReason>,
    #[serde(rename = "cancelReason", skip_serializing_if = "Option::is_none")]
    pub cancel_reason: Option<CancelReason>,
}

impl CiBuildRunAttributes {
    pub fn new() -> CiBuildRunAttributes {
        CiBuildRunAttributes {
            number: None,
            created_date: None,
            started_date: None,
            finished_date: None,
            source_commit: None,
            destination_commit: None,
            is_pull_request_build: None,
            issue_counts: None,
            execution_progress: None,
            completion_status: None,
            start_reason: None,
            cancel_reason: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum StartReason {
    #[serde(rename = "GIT_REF_CHANGE")]
    GitRefChange,
    #[serde(rename = "MANUAL")]
    Manual,
    #[serde(rename = "MANUAL_REBUILD")]
    ManualRebuild,
    #[serde(rename = "PULL_REQUEST_OPEN")]
    PullRequestOpen,
    #[serde(rename = "PULL_REQUEST_UPDATE")]
    PullRequestUpdate,
    #[serde(rename = "SCHEDULE")]
    Schedule,
}

impl Default for StartReason {
    fn default() -> StartReason {
        Self::GitRefChange
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CancelReason {
    #[serde(rename = "AUTOMATICALLY_BY_NEWER_BUILD")]
    AutomaticallyByNewerBuild,
    #[serde(rename = "MANUALLY_BY_USER")]
    ManuallyByUser,
}

impl Default for CancelReason {
    fn default() -> CancelReason {
        Self::AutomaticallyByNewerBuild
    }
}

