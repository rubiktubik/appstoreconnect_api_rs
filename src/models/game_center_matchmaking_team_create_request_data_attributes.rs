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
pub struct GameCenterMatchmakingTeamCreateRequestDataAttributes {
    #[serde(rename = "referenceName")]
    pub reference_name: String,
    #[serde(rename = "minPlayers")]
    pub min_players: i32,
    #[serde(rename = "maxPlayers")]
    pub max_players: i32,
}

impl GameCenterMatchmakingTeamCreateRequestDataAttributes {
    pub fn new(reference_name: String, min_players: i32, max_players: i32) -> GameCenterMatchmakingTeamCreateRequestDataAttributes {
        GameCenterMatchmakingTeamCreateRequestDataAttributes {
            reference_name,
            min_players,
            max_players,
        }
    }
}
