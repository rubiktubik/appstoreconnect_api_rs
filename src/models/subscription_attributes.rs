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
pub struct SubscriptionAttributes {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "productId", skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(rename = "familySharable", skip_serializing_if = "Option::is_none")]
    pub family_sharable: Option<bool>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
    #[serde(rename = "subscriptionPeriod", skip_serializing_if = "Option::is_none")]
    pub subscription_period: Option<SubscriptionPeriod>,
    #[serde(rename = "reviewNote", skip_serializing_if = "Option::is_none")]
    pub review_note: Option<String>,
    #[serde(rename = "groupLevel", skip_serializing_if = "Option::is_none")]
    pub group_level: Option<i32>,
}

impl SubscriptionAttributes {
    pub fn new() -> SubscriptionAttributes {
        SubscriptionAttributes {
            name: None,
            product_id: None,
            family_sharable: None,
            state: None,
            subscription_period: None,
            review_note: None,
            group_level: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "MISSING_METADATA")]
    MissingMetadata,
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

impl Default for State {
    fn default() -> State {
        Self::MissingMetadata
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SubscriptionPeriod {
    #[serde(rename = "ONE_WEEK")]
    OneWeek,
    #[serde(rename = "ONE_MONTH")]
    OneMonth,
    #[serde(rename = "TWO_MONTHS")]
    TwoMonths,
    #[serde(rename = "THREE_MONTHS")]
    ThreeMonths,
    #[serde(rename = "SIX_MONTHS")]
    SixMonths,
    #[serde(rename = "ONE_YEAR")]
    OneYear,
}

impl Default for SubscriptionPeriod {
    fn default() -> SubscriptionPeriod {
        Self::OneWeek
    }
}
