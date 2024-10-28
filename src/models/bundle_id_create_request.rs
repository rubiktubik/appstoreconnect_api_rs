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
pub struct BundleIdCreateRequest {
    #[serde(rename = "data")]
    pub data: Box<models::BundleIdCreateRequestData>,
}

impl BundleIdCreateRequest {
    pub fn new(data: models::BundleIdCreateRequestData) -> BundleIdCreateRequest {
        BundleIdCreateRequest {
            data: Box::new(data),
        }
    }
}

