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
pub struct GameCenterDetailUpdateRequestDataRelationships {
    #[serde(rename = "gameCenterGroup", skip_serializing_if = "Option::is_none")]
    pub game_center_group: Option<Box<models::GameCenterAchievementRelationshipsGameCenterGroup>>,
    #[serde(rename = "defaultLeaderboard", skip_serializing_if = "Option::is_none")]
    pub default_leaderboard: Option<Box<models::GameCenterDetailRelationshipsDefaultLeaderboard>>,
    #[serde(rename = "defaultGroupLeaderboard", skip_serializing_if = "Option::is_none")]
    pub default_group_leaderboard: Option<Box<models::GameCenterDetailRelationshipsDefaultLeaderboard>>,
}

impl GameCenterDetailUpdateRequestDataRelationships {
    pub fn new() -> GameCenterDetailUpdateRequestDataRelationships {
        GameCenterDetailUpdateRequestDataRelationships {
            game_center_group: None,
            default_leaderboard: None,
            default_group_leaderboard: None,
        }
    }
}
