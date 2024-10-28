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

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum InAppPurchaseState {
    #[serde(rename = "MISSING_METADATA")]
    MissingMetadata,
    #[serde(rename = "WAITING_FOR_UPLOAD")]
    WaitingForUpload,
    #[serde(rename = "PROCESSING_CONTENT")]
    ProcessingContent,
    #[serde(rename = "READY_TO_SUBMIT")]
    ReadyToSubmit,
    #[serde(rename = "WAITING_FOR_REVIEW")]
    WaitingForReview,
    #[serde(rename = "IN_REVIEW")]
    InReview,
    #[serde(rename = "DEVELOPER_ACTION_NEEDED")]
    DeveloperActionNeeded,
    #[serde(rename = "PENDING_BINARY_APPROVAL")]
    PendingBinaryApproval,
    #[serde(rename = "APPROVED")]
    Approved,
    #[serde(rename = "DEVELOPER_REMOVED_FROM_SALE")]
    DeveloperRemovedFromSale,
    #[serde(rename = "REMOVED_FROM_SALE")]
    RemovedFromSale,
    #[serde(rename = "REJECTED")]
    Rejected,

}

impl std::fmt::Display for InAppPurchaseState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::MissingMetadata => write!(f, "MISSING_METADATA"),
            Self::WaitingForUpload => write!(f, "WAITING_FOR_UPLOAD"),
            Self::ProcessingContent => write!(f, "PROCESSING_CONTENT"),
            Self::ReadyToSubmit => write!(f, "READY_TO_SUBMIT"),
            Self::WaitingForReview => write!(f, "WAITING_FOR_REVIEW"),
            Self::InReview => write!(f, "IN_REVIEW"),
            Self::DeveloperActionNeeded => write!(f, "DEVELOPER_ACTION_NEEDED"),
            Self::PendingBinaryApproval => write!(f, "PENDING_BINARY_APPROVAL"),
            Self::Approved => write!(f, "APPROVED"),
            Self::DeveloperRemovedFromSale => write!(f, "DEVELOPER_REMOVED_FROM_SALE"),
            Self::RemovedFromSale => write!(f, "REMOVED_FROM_SALE"),
            Self::Rejected => write!(f, "REJECTED"),
        }
    }
}

impl Default for InAppPurchaseState {
    fn default() -> InAppPurchaseState {
        Self::MissingMetadata
    }
}
