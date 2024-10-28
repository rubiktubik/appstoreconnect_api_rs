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
pub struct GameCenterEnabledVersionAttributes {
    #[serde(rename = "platform", skip_serializing_if = "Option::is_none")]
    pub platform: Option<models::Platform>,
    #[serde(rename = "versionString", skip_serializing_if = "Option::is_none")]
    pub version_string: Option<String>,
    #[serde(rename = "iconAsset", skip_serializing_if = "Option::is_none")]
    pub icon_asset: Option<Box<models::ImageAsset>>,
}

impl GameCenterEnabledVersionAttributes {
    pub fn new() -> GameCenterEnabledVersionAttributes {
        GameCenterEnabledVersionAttributes {
            platform: None,
            version_string: None,
            icon_asset: None,
        }
    }
}

