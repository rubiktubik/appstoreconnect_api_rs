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
pub struct GameCenterAppVersionCreateRequestData {
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "relationships")]
    pub relationships: Box<models::AlternativeDistributionPackageCreateRequestDataRelationships>,
}

impl GameCenterAppVersionCreateRequestData {
    pub fn new(r#type: Type, relationships: models::AlternativeDistributionPackageCreateRequestDataRelationships) -> GameCenterAppVersionCreateRequestData {
        GameCenterAppVersionCreateRequestData {
            r#type,
            relationships: Box::new(relationships),
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "gameCenterAppVersions")]
    GameCenterAppVersions,
}

impl Default for Type {
    fn default() -> Type {
        Self::GameCenterAppVersions
    }
}

