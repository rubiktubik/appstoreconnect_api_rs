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
pub struct GameCenterMatchmakingRuleSetTestAttributes {
    #[serde(rename = "matchmakingResults", skip_serializing_if = "Option::is_none")]
    pub matchmaking_results: Option<Vec<Vec<models::GameCenterMatchmakingRuleSetTestAttributesMatchmakingResultsInnerInner>>>,
}

impl GameCenterMatchmakingRuleSetTestAttributes {
    pub fn new() -> GameCenterMatchmakingRuleSetTestAttributes {
        GameCenterMatchmakingRuleSetTestAttributes {
            matchmaking_results: None,
        }
    }
}

