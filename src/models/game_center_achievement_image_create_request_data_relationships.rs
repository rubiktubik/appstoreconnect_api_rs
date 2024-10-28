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
pub struct GameCenterAchievementImageCreateRequestDataRelationships {
    #[serde(rename = "gameCenterAchievementLocalization")]
    pub game_center_achievement_localization: Box<models::GameCenterAchievementImageCreateRequestDataRelationshipsGameCenterAchievementLocalization>,
}

impl GameCenterAchievementImageCreateRequestDataRelationships {
    pub fn new(game_center_achievement_localization: models::GameCenterAchievementImageCreateRequestDataRelationshipsGameCenterAchievementLocalization) -> GameCenterAchievementImageCreateRequestDataRelationships {
        GameCenterAchievementImageCreateRequestDataRelationships {
            game_center_achievement_localization: Box::new(game_center_achievement_localization),
        }
    }
}
