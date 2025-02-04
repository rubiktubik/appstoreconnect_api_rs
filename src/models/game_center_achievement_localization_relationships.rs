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
pub struct GameCenterAchievementLocalizationRelationships {
    #[serde(rename = "gameCenterAchievement", skip_serializing_if = "Option::is_none")]
    pub game_center_achievement: Option<Box<models::GameCenterAchievementLocalizationRelationshipsGameCenterAchievement>>,
    #[serde(rename = "gameCenterAchievementImage", skip_serializing_if = "Option::is_none")]
    pub game_center_achievement_image: Option<Box<models::GameCenterAchievementLocalizationRelationshipsGameCenterAchievementImage>>,
}

impl GameCenterAchievementLocalizationRelationships {
    pub fn new() -> GameCenterAchievementLocalizationRelationships {
        GameCenterAchievementLocalizationRelationships {
            game_center_achievement: None,
            game_center_achievement_image: None,
        }
    }
}

