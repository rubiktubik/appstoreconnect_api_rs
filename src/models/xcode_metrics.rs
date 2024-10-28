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
pub struct XcodeMetrics {
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "insights", skip_serializing_if = "Option::is_none")]
    pub insights: Option<Box<models::XcodeMetricsInsights>>,
    #[serde(rename = "productData", skip_serializing_if = "Option::is_none")]
    pub product_data: Option<Vec<models::XcodeMetricsProductDataInner>>,
}

impl XcodeMetrics {
    pub fn new() -> XcodeMetrics {
        XcodeMetrics {
            version: None,
            insights: None,
            product_data: None,
        }
    }
}

