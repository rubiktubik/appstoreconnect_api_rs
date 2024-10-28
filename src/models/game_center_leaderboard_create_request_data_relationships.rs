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
pub struct GameCenterLeaderboardCreateRequestDataRelationships {
    #[serde(rename = "gameCenterDetail", skip_serializing_if = "Option::is_none")]
    pub game_center_detail: Option<Box<models::GameCenterAchievementReleaseRelationshipsGameCenterDetail>>,
    #[serde(rename = "gameCenterGroup", skip_serializing_if = "Option::is_none")]
    pub game_center_group: Option<Box<models::GameCenterAchievementRelationshipsGameCenterGroup>>,
    #[serde(rename = "gameCenterLeaderboardSets", skip_serializing_if = "Option::is_none")]
    pub game_center_leaderboard_sets: Option<Box<models::GameCenterLeaderboardCreateRequestDataRelationshipsGameCenterLeaderboardSets>>,
}

impl GameCenterLeaderboardCreateRequestDataRelationships {
    pub fn new() -> GameCenterLeaderboardCreateRequestDataRelationships {
        GameCenterLeaderboardCreateRequestDataRelationships {
            game_center_detail: None,
            game_center_group: None,
            game_center_leaderboard_sets: None,
        }
    }
}

