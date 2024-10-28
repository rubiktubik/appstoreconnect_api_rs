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
pub struct InAppPurchasePriceScheduleResponse {
    #[serde(rename = "data")]
    pub data: Box<models::InAppPurchasePriceSchedule>,
    #[serde(rename = "included", skip_serializing_if = "Option::is_none")]
    pub included: Option<Vec<models::InAppPurchasePriceScheduleResponseIncludedInner>>,
    #[serde(rename = "links")]
    pub links: Box<models::DocumentLinks>,
}

impl InAppPurchasePriceScheduleResponse {
    pub fn new(data: models::InAppPurchasePriceSchedule, links: models::DocumentLinks) -> InAppPurchasePriceScheduleResponse {
        InAppPurchasePriceScheduleResponse {
            data: Box::new(data),
            included: None,
            links: Box::new(links),
        }
    }
}
