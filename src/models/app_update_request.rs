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
pub struct AppUpdateRequest {
    #[serde(rename = "data")]
    pub data: Box<models::AppUpdateRequestData>,
}

impl AppUpdateRequest {
    pub fn new(data: models::AppUpdateRequestData) -> AppUpdateRequest {
        AppUpdateRequest {
            data: Box::new(data),
        }
    }
}
