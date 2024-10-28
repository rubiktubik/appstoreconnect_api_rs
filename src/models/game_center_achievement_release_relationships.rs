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
pub struct GameCenterAchievementReleaseRelationships {
    #[serde(rename = "gameCenterDetail", skip_serializing_if = "Option::is_none")]
    pub game_center_detail: Option<Box<models::GameCenterAchievementReleaseRelationshipsGameCenterDetail>>,
    #[serde(rename = "gameCenterAchievement", skip_serializing_if = "Option::is_none")]
    pub game_center_achievement: Option<Box<models::GameCenterAchievementReleaseRelationshipsGameCenterAchievement>>,
}

impl GameCenterAchievementReleaseRelationships {
    pub fn new() -> GameCenterAchievementReleaseRelationships {
        GameCenterAchievementReleaseRelationships {
            game_center_detail: None,
            game_center_achievement: None,
        }
    }
}

