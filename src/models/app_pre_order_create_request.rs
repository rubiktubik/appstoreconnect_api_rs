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
pub struct AppPreOrderCreateRequest {
    #[serde(rename = "data")]
    pub data: Box<models::AppPreOrderCreateRequestData>,
}

impl AppPreOrderCreateRequest {
    pub fn new(data: models::AppPreOrderCreateRequestData) -> AppPreOrderCreateRequest {
        AppPreOrderCreateRequest {
            data: Box::new(data),
        }
    }
}

