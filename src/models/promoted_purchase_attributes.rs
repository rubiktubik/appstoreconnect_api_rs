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
pub struct PromotedPurchaseAttributes {
    #[serde(rename = "visibleForAllUsers", skip_serializing_if = "Option::is_none")]
    pub visible_for_all_users: Option<bool>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
}

impl PromotedPurchaseAttributes {
    pub fn new() -> PromotedPurchaseAttributes {
        PromotedPurchaseAttributes {
            visible_for_all_users: None,
            enabled: None,
            state: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "APPROVED")]
    Approved,
    #[serde(rename = "IN_REVIEW")]
    InReview,
    #[serde(rename = "PREPARE_FOR_SUBMISSION")]
    PrepareForSubmission,
    #[serde(rename = "REJECTED")]
    Rejected,
}

impl Default for State {
    fn default() -> State {
        Self::Approved
    }
}
