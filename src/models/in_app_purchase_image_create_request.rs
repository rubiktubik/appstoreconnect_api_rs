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
pub struct InAppPurchaseImageCreateRequest {
    #[serde(rename = "data")]
    pub data: Box<models::InAppPurchaseImageCreateRequestData>,
}

impl InAppPurchaseImageCreateRequest {
    pub fn new(data: models::InAppPurchaseImageCreateRequestData) -> InAppPurchaseImageCreateRequest {
        InAppPurchaseImageCreateRequest {
            data: Box::new(data),
        }
    }
}

