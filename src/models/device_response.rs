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
pub struct DeviceResponse {
    #[serde(rename = "data")]
    pub data: Box<models::Device>,
    #[serde(rename = "links")]
    pub links: Box<models::DocumentLinks>,
}

impl DeviceResponse {
    pub fn new(data: models::Device, links: models::DocumentLinks) -> DeviceResponse {
        DeviceResponse {
            data: Box::new(data),
            links: Box::new(links),
        }
    }
}

