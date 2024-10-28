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
pub struct GameCenterLeaderboardLocalizationCreateRequestDataAttributes {
    #[serde(rename = "locale")]
    pub locale: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "formatterOverride", skip_serializing_if = "Option::is_none")]
    pub formatter_override: Option<models::GameCenterLeaderboardFormatter>,
    #[serde(rename = "formatterSuffix", skip_serializing_if = "Option::is_none")]
    pub formatter_suffix: Option<String>,
    #[serde(rename = "formatterSuffixSingular", skip_serializing_if = "Option::is_none")]
    pub formatter_suffix_singular: Option<String>,
}

impl GameCenterLeaderboardLocalizationCreateRequestDataAttributes {
    pub fn new(locale: String, name: String) -> GameCenterLeaderboardLocalizationCreateRequestDataAttributes {
        GameCenterLeaderboardLocalizationCreateRequestDataAttributes {
            locale,
            name,
            formatter_override: None,
            formatter_suffix: None,
            formatter_suffix_singular: None,
        }
    }
}

