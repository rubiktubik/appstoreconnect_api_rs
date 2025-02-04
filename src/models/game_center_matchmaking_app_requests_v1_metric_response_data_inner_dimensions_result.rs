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
pub struct GameCenterMatchmakingAppRequestsV1MetricResponseDataInnerDimensionsResult {
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<Box<models::GameCenterMatchmakingAppRequestsV1MetricResponseDataInnerDimensionsResultLinks>>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Data>,
}

impl GameCenterMatchmakingAppRequestsV1MetricResponseDataInnerDimensionsResult {
    pub fn new() -> GameCenterMatchmakingAppRequestsV1MetricResponseDataInnerDimensionsResult {
        GameCenterMatchmakingAppRequestsV1MetricResponseDataInnerDimensionsResult {
            links: None,
            data: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Data {
    #[serde(rename = "MATCHED")]
    Matched,
    #[serde(rename = "CANCELED")]
    Canceled,
    #[serde(rename = "EXPIRED")]
    Expired,
}

impl Default for Data {
    fn default() -> Data {
        Self::Matched
    }
}

