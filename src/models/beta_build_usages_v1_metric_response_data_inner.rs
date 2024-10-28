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
pub struct BetaBuildUsagesV1MetricResponseDataInner {
    #[serde(rename = "dataPoints", skip_serializing_if = "Option::is_none")]
    pub data_points: Option<Box<models::BetaBuildUsagesV1MetricResponseDataInnerDataPoints>>,
}

impl BetaBuildUsagesV1MetricResponseDataInner {
    pub fn new() -> BetaBuildUsagesV1MetricResponseDataInner {
        BetaBuildUsagesV1MetricResponseDataInner {
            data_points: None,
        }
    }
}

