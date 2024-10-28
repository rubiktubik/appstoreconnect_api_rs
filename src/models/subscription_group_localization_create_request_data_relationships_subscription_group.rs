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
pub struct SubscriptionGroupLocalizationCreateRequestDataRelationshipsSubscriptionGroup {
    #[serde(rename = "data")]
    pub data: Box<models::AppRelationshipsSubscriptionGroupsDataInner>,
}

impl SubscriptionGroupLocalizationCreateRequestDataRelationshipsSubscriptionGroup {
    pub fn new(data: models::AppRelationshipsSubscriptionGroupsDataInner) -> SubscriptionGroupLocalizationCreateRequestDataRelationshipsSubscriptionGroup {
        SubscriptionGroupLocalizationCreateRequestDataRelationshipsSubscriptionGroup {
            data: Box::new(data),
        }
    }
}
