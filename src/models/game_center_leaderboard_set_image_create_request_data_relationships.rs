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
pub struct GameCenterLeaderboardSetImageCreateRequestDataRelationships {
    #[serde(rename = "gameCenterLeaderboardSetLocalization")]
    pub game_center_leaderboard_set_localization: Box<models::GameCenterLeaderboardSetImageCreateRequestDataRelationshipsGameCenterLeaderboardSetLocalization>,
}

impl GameCenterLeaderboardSetImageCreateRequestDataRelationships {
    pub fn new(game_center_leaderboard_set_localization: models::GameCenterLeaderboardSetImageCreateRequestDataRelationshipsGameCenterLeaderboardSetLocalization) -> GameCenterLeaderboardSetImageCreateRequestDataRelationships {
        GameCenterLeaderboardSetImageCreateRequestDataRelationships {
            game_center_leaderboard_set_localization: Box::new(game_center_leaderboard_set_localization),
        }
    }
}
