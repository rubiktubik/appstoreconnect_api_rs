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
pub struct GameCenterMatchmakingQueueRequestsV1MetricResponseDataInner {
    #[serde(rename = "dataPoints", skip_serializing_if = "Option::is_none")]
    pub data_points: Option<Box<models::GameCenterMatchmakingAppRequestsV1MetricResponseDataInnerDataPoints>>,
    #[serde(rename = "dimensions", skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Box<models::GameCenterMatchmakingQueueRequestsV1MetricResponseDataInnerDimensions>>,
    #[serde(rename = "granularity", skip_serializing_if = "Option::is_none")]
    pub granularity: Option<Granularity>,
}

impl GameCenterMatchmakingQueueRequestsV1MetricResponseDataInner {
    pub fn new() -> GameCenterMatchmakingQueueRequestsV1MetricResponseDataInner {
        GameCenterMatchmakingQueueRequestsV1MetricResponseDataInner {
            data_points: None,
            dimensions: None,
            granularity: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Granularity {
    #[serde(rename = "P1D")]
    P1D,
    #[serde(rename = "PT1H")]
    Pt1H,
    #[serde(rename = "PT15M")]
    Pt15M,
}

impl Default for Granularity {
    fn default() -> Granularity {
        Self::P1D
    }
}

