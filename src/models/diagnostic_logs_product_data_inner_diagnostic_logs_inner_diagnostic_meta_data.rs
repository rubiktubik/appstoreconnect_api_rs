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
pub struct DiagnosticLogsProductDataInnerDiagnosticLogsInnerDiagnosticMetaData {
    #[serde(rename = "bundleId", skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    #[serde(rename = "event", skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
    #[serde(rename = "osVersion", skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    #[serde(rename = "appVersion", skip_serializing_if = "Option::is_none")]
    pub app_version: Option<String>,
    #[serde(rename = "writesCaused", skip_serializing_if = "Option::is_none")]
    pub writes_caused: Option<String>,
    #[serde(rename = "deviceType", skip_serializing_if = "Option::is_none")]
    pub device_type: Option<String>,
    #[serde(rename = "platformArchitecture", skip_serializing_if = "Option::is_none")]
    pub platform_architecture: Option<String>,
    #[serde(rename = "eventDetail", skip_serializing_if = "Option::is_none")]
    pub event_detail: Option<String>,
    #[serde(rename = "buildVersion", skip_serializing_if = "Option::is_none")]
    pub build_version: Option<String>,
}

impl DiagnosticLogsProductDataInnerDiagnosticLogsInnerDiagnosticMetaData {
    pub fn new() -> DiagnosticLogsProductDataInnerDiagnosticLogsInnerDiagnosticMetaData {
        DiagnosticLogsProductDataInnerDiagnosticLogsInnerDiagnosticMetaData {
            bundle_id: None,
            event: None,
            os_version: None,
            app_version: None,
            writes_caused: None,
            device_type: None,
            platform_architecture: None,
            event_detail: None,
            build_version: None,
        }
    }
}

