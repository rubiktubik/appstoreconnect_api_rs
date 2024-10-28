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
pub struct GameCenterAchievementAttributes {
    #[serde(rename = "referenceName", skip_serializing_if = "Option::is_none")]
    pub reference_name: Option<String>,
    #[serde(rename = "vendorIdentifier", skip_serializing_if = "Option::is_none")]
    pub vendor_identifier: Option<String>,
    #[serde(rename = "points", skip_serializing_if = "Option::is_none")]
    pub points: Option<i32>,
    #[serde(rename = "showBeforeEarned", skip_serializing_if = "Option::is_none")]
    pub show_before_earned: Option<bool>,
    #[serde(rename = "repeatable", skip_serializing_if = "Option::is_none")]
    pub repeatable: Option<bool>,
    #[serde(rename = "archived", skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
}

impl GameCenterAchievementAttributes {
    pub fn new() -> GameCenterAchievementAttributes {
        GameCenterAchievementAttributes {
            reference_name: None,
            vendor_identifier: None,
            points: None,
            show_before_earned: None,
            repeatable: None,
            archived: None,
        }
    }
}

