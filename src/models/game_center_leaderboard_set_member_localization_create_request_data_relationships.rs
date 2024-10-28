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
pub struct GameCenterLeaderboardSetMemberLocalizationCreateRequestDataRelationships {
    #[serde(rename = "gameCenterLeaderboardSet")]
    pub game_center_leaderboard_set: Box<models::GameCenterLeaderboardSetLocalizationCreateRequestDataRelationshipsGameCenterLeaderboardSet>,
    #[serde(rename = "gameCenterLeaderboard")]
    pub game_center_leaderboard: Box<models::GameCenterLeaderboardLocalizationCreateRequestDataRelationshipsGameCenterLeaderboard>,
}

impl GameCenterLeaderboardSetMemberLocalizationCreateRequestDataRelationships {
    pub fn new(game_center_leaderboard_set: models::GameCenterLeaderboardSetLocalizationCreateRequestDataRelationshipsGameCenterLeaderboardSet, game_center_leaderboard: models::GameCenterLeaderboardLocalizationCreateRequestDataRelationshipsGameCenterLeaderboard) -> GameCenterLeaderboardSetMemberLocalizationCreateRequestDataRelationships {
        GameCenterLeaderboardSetMemberLocalizationCreateRequestDataRelationships {
            game_center_leaderboard_set: Box::new(game_center_leaderboard_set),
            game_center_leaderboard: Box::new(game_center_leaderboard),
        }
    }
}
