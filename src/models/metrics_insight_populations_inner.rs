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
pub struct MetricsInsightPopulationsInner {
    #[serde(rename = "deltaPercentage", skip_serializing_if = "Option::is_none")]
    pub delta_percentage: Option<f64>,
    #[serde(rename = "percentile", skip_serializing_if = "Option::is_none")]
    pub percentile: Option<String>,
    #[serde(rename = "summaryString", skip_serializing_if = "Option::is_none")]
    pub summary_string: Option<String>,
    #[serde(rename = "referenceAverageValue", skip_serializing_if = "Option::is_none")]
    pub reference_average_value: Option<f64>,
    #[serde(rename = "latestVersionValue", skip_serializing_if = "Option::is_none")]
    pub latest_version_value: Option<f64>,
    #[serde(rename = "device", skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
}

impl MetricsInsightPopulationsInner {
    pub fn new() -> MetricsInsightPopulationsInner {
        MetricsInsightPopulationsInner {
            delta_percentage: None,
            percentile: None,
            summary_string: None,
            reference_average_value: None,
            latest_version_value: None,
            device: None,
        }
    }
}
