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
pub struct AppAvailabilityCreateRequest {
    #[serde(rename = "data")]
    pub data: Box<models::AppAvailabilityCreateRequestData>,
}

impl AppAvailabilityCreateRequest {
    pub fn new(data: models::AppAvailabilityCreateRequestData) -> AppAvailabilityCreateRequest {
        AppAvailabilityCreateRequest {
            data: Box::new(data),
        }
    }
}

