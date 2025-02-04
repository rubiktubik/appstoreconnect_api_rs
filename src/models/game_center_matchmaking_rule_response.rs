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
pub struct GameCenterMatchmakingRuleResponse {
    #[serde(rename = "data")]
    pub data: Box<models::GameCenterMatchmakingRule>,
    #[serde(rename = "links")]
    pub links: Box<models::DocumentLinks>,
}

impl GameCenterMatchmakingRuleResponse {
    pub fn new(data: models::GameCenterMatchmakingRule, links: models::DocumentLinks) -> GameCenterMatchmakingRuleResponse {
        GameCenterMatchmakingRuleResponse {
            data: Box::new(data),
            links: Box::new(links),
        }
    }
}

