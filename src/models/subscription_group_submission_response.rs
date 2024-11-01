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
pub struct SubscriptionGroupSubmissionResponse {
    #[serde(rename = "data")]
    pub data: Box<models::SubscriptionGroupSubmission>,
    #[serde(rename = "links")]
    pub links: Box<models::DocumentLinks>,
}

impl SubscriptionGroupSubmissionResponse {
    pub fn new(data: models::SubscriptionGroupSubmission, links: models::DocumentLinks) -> SubscriptionGroupSubmissionResponse {
        SubscriptionGroupSubmissionResponse {
            data: Box::new(data),
            links: Box::new(links),
        }
    }
}

