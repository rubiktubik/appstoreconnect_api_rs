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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GameCenterDetailsResponseIncludedInner {
    App(Box<models::App>),
    GameCenterAppVersion(Box<models::GameCenterAppVersion>),
    GameCenterGroup(Box<models::GameCenterGroup>),
    GameCenterLeaderboard(Box<models::GameCenterLeaderboard>),
    GameCenterLeaderboardSet(Box<models::GameCenterLeaderboardSet>),
    GameCenterAchievement(Box<models::GameCenterAchievement>),
    GameCenterAchievementRelease(Box<models::GameCenterAchievementRelease>),
    GameCenterLeaderboardRelease(Box<models::GameCenterLeaderboardRelease>),
    GameCenterLeaderboardSetRelease(Box<models::GameCenterLeaderboardSetRelease>),
}

impl Default for GameCenterDetailsResponseIncludedInner {
    fn default() -> Self {
        Self::App(Default::default())
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "apps")]
    Apps,
    #[serde(rename = "gameCenterAppVersions")]
    GameCenterAppVersions,
    #[serde(rename = "gameCenterGroups")]
    GameCenterGroups,
    #[serde(rename = "gameCenterLeaderboards")]
    GameCenterLeaderboards,
    #[serde(rename = "gameCenterLeaderboardSets")]
    GameCenterLeaderboardSets,
    #[serde(rename = "gameCenterAchievements")]
    GameCenterAchievements,
    #[serde(rename = "gameCenterAchievementReleases")]
    GameCenterAchievementReleases,
    #[serde(rename = "gameCenterLeaderboardReleases")]
    GameCenterLeaderboardReleases,
    #[serde(rename = "gameCenterLeaderboardSetReleases")]
    GameCenterLeaderboardSetReleases,
}

impl Default for Type {
    fn default() -> Type {
        Self::Apps
    }
}

