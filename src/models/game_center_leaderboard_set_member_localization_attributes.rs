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
pub struct GameCenterLeaderboardSetMemberLocalizationAttributes {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "locale", skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
}

impl GameCenterLeaderboardSetMemberLocalizationAttributes {
    pub fn new() -> GameCenterLeaderboardSetMemberLocalizationAttributes {
        GameCenterLeaderboardSetMemberLocalizationAttributes {
            name: None,
            locale: None,
        }
    }
}

