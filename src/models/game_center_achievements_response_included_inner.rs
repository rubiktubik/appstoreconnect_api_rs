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
pub enum GameCenterAchievementsResponseIncludedInner {
    GameCenterDetail(Box<models::GameCenterDetail>),
    GameCenterGroup(Box<models::GameCenterGroup>),
    GameCenterAchievement(Box<models::GameCenterAchievement>),
    GameCenterAchievementLocalization(Box<models::GameCenterAchievementLocalization>),
    GameCenterAchievementRelease(Box<models::GameCenterAchievementRelease>),
}

impl Default for GameCenterAchievementsResponseIncludedInner {
    fn default() -> Self {
        Self::GameCenterDetail(Default::default())
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "gameCenterDetails")]
    GameCenterDetails,
    #[serde(rename = "gameCenterGroups")]
    GameCenterGroups,
    #[serde(rename = "gameCenterAchievements")]
    GameCenterAchievements,
    #[serde(rename = "gameCenterAchievementLocalizations")]
    GameCenterAchievementLocalizations,
    #[serde(rename = "gameCenterAchievementReleases")]
    GameCenterAchievementReleases,
}

impl Default for Type {
    fn default() -> Type {
        Self::GameCenterDetails
    }
}
