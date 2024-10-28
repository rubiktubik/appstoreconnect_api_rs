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
pub struct PerfPowerMetricAttributes {
    #[serde(rename = "platform", skip_serializing_if = "Option::is_none")]
    pub platform: Option<Platform>,
    #[serde(rename = "metricType", skip_serializing_if = "Option::is_none")]
    pub metric_type: Option<MetricType>,
    #[serde(rename = "deviceType", skip_serializing_if = "Option::is_none")]
    pub device_type: Option<String>,
}

impl PerfPowerMetricAttributes {
    pub fn new() -> PerfPowerMetricAttributes {
        PerfPowerMetricAttributes {
            platform: None,
            metric_type: None,
            device_type: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Platform {
    #[serde(rename = "IOS")]
    Ios,
}

impl Default for Platform {
    fn default() -> Platform {
        Self::Ios
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MetricType {
    #[serde(rename = "DISK")]
    Disk,
    #[serde(rename = "HANG")]
    Hang,
    #[serde(rename = "BATTERY")]
    Battery,
    #[serde(rename = "LAUNCH")]
    Launch,
    #[serde(rename = "MEMORY")]
    Memory,
    #[serde(rename = "ANIMATION")]
    Animation,
    #[serde(rename = "TERMINATION")]
    Termination,
}

impl Default for MetricType {
    fn default() -> MetricType {
        Self::Disk
    }
}

