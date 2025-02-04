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
pub struct GameCenterLeaderboardLocalizationRelationships {
    #[serde(rename = "gameCenterLeaderboard", skip_serializing_if = "Option::is_none")]
    pub game_center_leaderboard: Option<Box<models::GameCenterDetailRelationshipsDefaultLeaderboard>>,
    #[serde(rename = "gameCenterLeaderboardImage", skip_serializing_if = "Option::is_none")]
    pub game_center_leaderboard_image: Option<Box<models::GameCenterLeaderboardLocalizationRelationshipsGameCenterLeaderboardImage>>,
}

impl GameCenterLeaderboardLocalizationRelationships {
    pub fn new() -> GameCenterLeaderboardLocalizationRelationships {
        GameCenterLeaderboardLocalizationRelationships {
            game_center_leaderboard: None,
            game_center_leaderboard_image: None,
        }
    }
}

