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
pub struct GameCenterAchievementCreateRequestData {
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "attributes")]
    pub attributes: Box<models::GameCenterAchievementCreateRequestDataAttributes>,
    #[serde(rename = "relationships", skip_serializing_if = "Option::is_none")]
    pub relationships: Option<Box<models::GameCenterAchievementCreateRequestDataRelationships>>,
}

impl GameCenterAchievementCreateRequestData {
    pub fn new(r#type: Type, attributes: models::GameCenterAchievementCreateRequestDataAttributes) -> GameCenterAchievementCreateRequestData {
        GameCenterAchievementCreateRequestData {
            r#type,
            attributes: Box::new(attributes),
            relationships: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "gameCenterAchievements")]
    GameCenterAchievements,
}

impl Default for Type {
    fn default() -> Type {
        Self::GameCenterAchievements
    }
}
