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
pub struct WinBackOfferAttributes {
    #[serde(rename = "referenceName", skip_serializing_if = "Option::is_none")]
    pub reference_name: Option<String>,
    #[serde(rename = "offerId", skip_serializing_if = "Option::is_none")]
    pub offer_id: Option<String>,
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<models::SubscriptionOfferDuration>,
    #[serde(rename = "offerMode", skip_serializing_if = "Option::is_none")]
    pub offer_mode: Option<models::SubscriptionOfferMode>,
    #[serde(rename = "periodCount", skip_serializing_if = "Option::is_none")]
    pub period_count: Option<i32>,
    #[serde(rename = "customerEligibilityPaidSubscriptionDurationInMonths", skip_serializing_if = "Option::is_none")]
    pub customer_eligibility_paid_subscription_duration_in_months: Option<i32>,
    #[serde(rename = "customerEligibilityTimeSinceLastSubscribedInMonths", skip_serializing_if = "Option::is_none")]
    pub customer_eligibility_time_since_last_subscribed_in_months: Option<Box<models::IntegerRange>>,
    #[serde(rename = "customerEligibilityWaitBetweenOffersInMonths", skip_serializing_if = "Option::is_none")]
    pub customer_eligibility_wait_between_offers_in_months: Option<i32>,
    #[serde(rename = "startDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(rename = "endDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<Priority>,
    #[serde(rename = "promotionIntent", skip_serializing_if = "Option::is_none")]
    pub promotion_intent: Option<PromotionIntent>,
}

impl WinBackOfferAttributes {
    pub fn new() -> WinBackOfferAttributes {
        WinBackOfferAttributes {
            reference_name: None,
            offer_id: None,
            duration: None,
            offer_mode: None,
            period_count: None,
            customer_eligibility_paid_subscription_duration_in_months: None,
            customer_eligibility_time_since_last_subscribed_in_months: None,
            customer_eligibility_wait_between_offers_in_months: None,
            start_date: None,
            end_date: None,
            priority: None,
            promotion_intent: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Priority {
    #[serde(rename = "HIGH")]
    High,
    #[serde(rename = "NORMAL")]
    Normal,
}

impl Default for Priority {
    fn default() -> Priority {
        Self::High
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PromotionIntent {
    #[serde(rename = "NOT_PROMOTED")]
    NotPromoted,
    #[serde(rename = "USE_AUTO_GENERATED_ASSETS")]
    UseAutoGeneratedAssets,
}

impl Default for PromotionIntent {
    fn default() -> PromotionIntent {
        Self::NotPromoted
    }
}

