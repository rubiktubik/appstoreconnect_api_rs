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
pub struct GameCenterAchievementLocalizationUpdateRequestDataAttributes {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "beforeEarnedDescription", skip_serializing_if = "Option::is_none")]
    pub before_earned_description: Option<String>,
    #[serde(rename = "afterEarnedDescription", skip_serializing_if = "Option::is_none")]
    pub after_earned_description: Option<String>,
}

impl GameCenterAchievementLocalizationUpdateRequestDataAttributes {
    pub fn new() -> GameCenterAchievementLocalizationUpdateRequestDataAttributes {
        GameCenterAchievementLocalizationUpdateRequestDataAttributes {
            name: None,
            before_earned_description: None,
            after_earned_description: None,
        }
    }
}
