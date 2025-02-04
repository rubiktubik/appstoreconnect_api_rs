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
pub struct InAppPurchaseAttributes {
    #[serde(rename = "referenceName", skip_serializing_if = "Option::is_none")]
    pub reference_name: Option<String>,
    #[serde(rename = "productId", skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(rename = "inAppPurchaseType", skip_serializing_if = "Option::is_none")]
    pub in_app_purchase_type: Option<InAppPurchaseType>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
}

impl InAppPurchaseAttributes {
    pub fn new() -> InAppPurchaseAttributes {
        InAppPurchaseAttributes {
            reference_name: None,
            product_id: None,
            in_app_purchase_type: None,
            state: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum InAppPurchaseType {
    #[serde(rename = "AUTOMATICALLY_RENEWABLE_SUBSCRIPTION")]
    AutomaticallyRenewableSubscription,
    #[serde(rename = "NON_CONSUMABLE")]
    NonConsumable,
    #[serde(rename = "CONSUMABLE")]
    Consumable,
    #[serde(rename = "NON_RENEWING_SUBSCRIPTION")]
    NonRenewingSubscription,
    #[serde(rename = "FREE_SUBSCRIPTION")]
    FreeSubscription,
}

impl Default for InAppPurchaseType {
    fn default() -> InAppPurchaseType {
        Self::AutomaticallyRenewableSubscription
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "CREATED")]
    Created,
    #[serde(rename = "DEVELOPER_SIGNED_OFF")]
    DeveloperSignedOff,
    #[serde(rename = "DEVELOPER_ACTION_NEEDED")]
    DeveloperActionNeeded,
    #[serde(rename = "DELETION_IN_PROGRESS")]
    DeletionInProgress,
    #[serde(rename = "APPROVED")]
    Approved,
    #[serde(rename = "DELETED")]
    Deleted,
    #[serde(rename = "REMOVED_FROM_SALE")]
    RemovedFromSale,
    #[serde(rename = "DEVELOPER_REMOVED_FROM_SALE")]
    DeveloperRemovedFromSale,
    #[serde(rename = "WAITING_FOR_UPLOAD")]
    WaitingForUpload,
    #[serde(rename = "PROCESSING_CONTENT")]
    ProcessingContent,
    #[serde(rename = "REPLACED")]
    Replaced,
    #[serde(rename = "REJECTED")]
    Rejected,
    #[serde(rename = "WAITING_FOR_SCREENSHOT")]
    WaitingForScreenshot,
    #[serde(rename = "PREPARE_FOR_SUBMISSION")]
    PrepareForSubmission,
    #[serde(rename = "MISSING_METADATA")]
    MissingMetadata,
    #[serde(rename = "READY_TO_SUBMIT")]
    ReadyToSubmit,
    #[serde(rename = "WAITING_FOR_REVIEW")]
    WaitingForReview,
    #[serde(rename = "IN_REVIEW")]
    InReview,
    #[serde(rename = "PENDING_DEVELOPER_RELEASE")]
    PendingDeveloperRelease,
}

impl Default for State {
    fn default() -> State {
        Self::Created
    }
}

