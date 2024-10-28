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
pub struct DiagnosticLogsProductDataInnerDiagnosticInsightsInner {
    #[serde(rename = "insightsURL", skip_serializing_if = "Option::is_none")]
    pub insights_url: Option<String>,
    #[serde(rename = "insightsCategory", skip_serializing_if = "Option::is_none")]
    pub insights_category: Option<String>,
    #[serde(rename = "insightsString", skip_serializing_if = "Option::is_none")]
    pub insights_string: Option<String>,
}

impl DiagnosticLogsProductDataInnerDiagnosticInsightsInner {
    pub fn new() -> DiagnosticLogsProductDataInnerDiagnosticInsightsInner {
        DiagnosticLogsProductDataInnerDiagnosticInsightsInner {
            insights_url: None,
            insights_category: None,
            insights_string: None,
        }
    }
}

