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
pub enum GameCenterEnabledVersionsResponseIncludedInner {
    GameCenterEnabledVersion(Box<models::GameCenterEnabledVersion>),
    App(Box<models::App>),
}

impl Default for GameCenterEnabledVersionsResponseIncludedInner {
    fn default() -> Self {
        Self::GameCenterEnabledVersion(Default::default())
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "gameCenterEnabledVersions")]
    GameCenterEnabledVersions,
    #[serde(rename = "apps")]
    Apps,
}

impl Default for Type {
    fn default() -> Type {
        Self::GameCenterEnabledVersions
    }
}
