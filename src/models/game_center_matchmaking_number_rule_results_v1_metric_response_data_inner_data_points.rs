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
pub struct GameCenterMatchmakingNumberRuleResultsV1MetricResponseDataInnerDataPoints {
    #[serde(rename = "start", skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    #[serde(rename = "values", skip_serializing_if = "Option::is_none")]
    pub values: Option<Box<models::GameCenterMatchmakingNumberRuleResultsV1MetricResponseDataInnerDataPointsValues>>,
}

impl GameCenterMatchmakingNumberRuleResultsV1MetricResponseDataInnerDataPoints {
    pub fn new() -> GameCenterMatchmakingNumberRuleResultsV1MetricResponseDataInnerDataPoints {
        GameCenterMatchmakingNumberRuleResultsV1MetricResponseDataInnerDataPoints {
            start: None,
            end: None,
            values: None,
        }
    }
}

