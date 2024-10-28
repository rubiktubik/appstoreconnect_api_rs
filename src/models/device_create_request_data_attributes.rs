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
pub struct DeviceCreateRequestDataAttributes {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "platform")]
    pub platform: models::BundleIdPlatform,
    #[serde(rename = "udid")]
    pub udid: String,
}

impl DeviceCreateRequestDataAttributes {
    pub fn new(name: String, platform: models::BundleIdPlatform, udid: String) -> DeviceCreateRequestDataAttributes {
        DeviceCreateRequestDataAttributes {
            name,
            platform,
            udid,
        }
    }
}

