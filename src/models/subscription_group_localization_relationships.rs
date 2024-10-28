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
pub struct SubscriptionGroupLocalizationRelationships {
    #[serde(rename = "subscriptionGroup", skip_serializing_if = "Option::is_none")]
    pub subscription_group: Option<Box<models::SubscriptionGroupLocalizationRelationshipsSubscriptionGroup>>,
}

impl SubscriptionGroupLocalizationRelationships {
    pub fn new() -> SubscriptionGroupLocalizationRelationships {
        SubscriptionGroupLocalizationRelationships {
            subscription_group: None,
        }
    }
}
