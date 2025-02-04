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
pub struct GameCenterEnabledVersionRelationships {
    #[serde(rename = "compatibleVersions", skip_serializing_if = "Option::is_none")]
    pub compatible_versions: Option<Box<models::AppRelationshipsGameCenterEnabledVersions>>,
    #[serde(rename = "app", skip_serializing_if = "Option::is_none")]
    pub app: Option<Box<models::AlternativeDistributionKeyCreateRequestDataRelationshipsApp>>,
}

impl GameCenterEnabledVersionRelationships {
    pub fn new() -> GameCenterEnabledVersionRelationships {
        GameCenterEnabledVersionRelationships {
            compatible_versions: None,
            app: None,
        }
    }
}

