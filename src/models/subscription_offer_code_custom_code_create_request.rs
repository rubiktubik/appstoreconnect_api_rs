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
pub struct SubscriptionOfferCodeCustomCodeCreateRequest {
    #[serde(rename = "data")]
    pub data: Box<models::SubscriptionOfferCodeCustomCodeCreateRequestData>,
}

impl SubscriptionOfferCodeCustomCodeCreateRequest {
    pub fn new(data: models::SubscriptionOfferCodeCustomCodeCreateRequestData) -> SubscriptionOfferCodeCustomCodeCreateRequest {
        SubscriptionOfferCodeCustomCodeCreateRequest {
            data: Box::new(data),
        }
    }
}

