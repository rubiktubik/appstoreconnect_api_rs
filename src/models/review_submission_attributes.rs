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
pub struct ReviewSubmissionAttributes {
    #[serde(rename = "platform", skip_serializing_if = "Option::is_none")]
    pub platform: Option<models::Platform>,
    #[serde(rename = "submittedDate", skip_serializing_if = "Option::is_none")]
    pub submitted_date: Option<String>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
}

impl ReviewSubmissionAttributes {
    pub fn new() -> ReviewSubmissionAttributes {
        ReviewSubmissionAttributes {
            platform: None,
            submitted_date: None,
            state: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "READY_FOR_REVIEW")]
    ReadyForReview,
    #[serde(rename = "WAITING_FOR_REVIEW")]
    WaitingForReview,
    #[serde(rename = "IN_REVIEW")]
    InReview,
    #[serde(rename = "UNRESOLVED_ISSUES")]
    UnresolvedIssues,
    #[serde(rename = "CANCELING")]
    Canceling,
    #[serde(rename = "COMPLETING")]
    Completing,
    #[serde(rename = "COMPLETE")]
    Complete,
}

impl Default for State {
    fn default() -> State {
        Self::ReadyForReview
    }
}
