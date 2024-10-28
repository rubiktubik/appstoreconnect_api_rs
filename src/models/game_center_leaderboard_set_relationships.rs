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
pub struct GameCenterLeaderboardSetRelationships {
    #[serde(rename = "gameCenterDetail", skip_serializing_if = "Option::is_none")]
    pub game_center_detail: Option<Box<models::GameCenterAchievementReleaseRelationshipsGameCenterDetail>>,
    #[serde(rename = "gameCenterGroup", skip_serializing_if = "Option::is_none")]
    pub game_center_group: Option<Box<models::GameCenterAchievementRelationshipsGameCenterGroup>>,
    #[serde(rename = "groupLeaderboardSet", skip_serializing_if = "Option::is_none")]
    pub group_leaderboard_set: Option<Box<models::GameCenterLeaderboardSetMemberLocalizationRelationshipsGameCenterLeaderboardSet>>,
    #[serde(rename = "localizations", skip_serializing_if = "Option::is_none")]
    pub localizations: Option<Box<models::GameCenterLeaderboardSetRelationshipsLocalizations>>,
    #[serde(rename = "gameCenterLeaderboards", skip_serializing_if = "Option::is_none")]
    pub game_center_leaderboards: Option<Box<models::GameCenterDetailRelationshipsGameCenterLeaderboards>>,
    #[serde(rename = "releases", skip_serializing_if = "Option::is_none")]
    pub releases: Option<Box<models::GameCenterDetailRelationshipsLeaderboardSetReleases>>,
}

impl GameCenterLeaderboardSetRelationships {
    pub fn new() -> GameCenterLeaderboardSetRelationships {
        GameCenterLeaderboardSetRelationships {
            game_center_detail: None,
            game_center_group: None,
            group_leaderboard_set: None,
            localizations: None,
            game_center_leaderboards: None,
            releases: None,
        }
    }
}

