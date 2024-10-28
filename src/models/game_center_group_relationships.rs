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
pub struct GameCenterGroupRelationships {
    #[serde(rename = "gameCenterDetails", skip_serializing_if = "Option::is_none")]
    pub game_center_details: Option<Box<models::GameCenterGroupRelationshipsGameCenterDetails>>,
    #[serde(rename = "gameCenterLeaderboards", skip_serializing_if = "Option::is_none")]
    pub game_center_leaderboards: Option<Box<models::GameCenterDetailRelationshipsGameCenterLeaderboards>>,
    #[serde(rename = "gameCenterLeaderboardSets", skip_serializing_if = "Option::is_none")]
    pub game_center_leaderboard_sets: Option<Box<models::GameCenterDetailRelationshipsGameCenterLeaderboardSets>>,
    #[serde(rename = "gameCenterAchievements", skip_serializing_if = "Option::is_none")]
    pub game_center_achievements: Option<Box<models::GameCenterDetailRelationshipsGameCenterAchievements>>,
}

impl GameCenterGroupRelationships {
    pub fn new() -> GameCenterGroupRelationships {
        GameCenterGroupRelationships {
            game_center_details: None,
            game_center_leaderboards: None,
            game_center_leaderboard_sets: None,
            game_center_achievements: None,
        }
    }
}

