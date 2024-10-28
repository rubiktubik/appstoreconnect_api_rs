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
pub struct AnalyticsReportRequestAttributes {
    #[serde(rename = "accessType", skip_serializing_if = "Option::is_none")]
    pub access_type: Option<AccessType>,
    #[serde(rename = "stoppedDueToInactivity", skip_serializing_if = "Option::is_none")]
    pub stopped_due_to_inactivity: Option<bool>,
}

impl AnalyticsReportRequestAttributes {
    pub fn new() -> AnalyticsReportRequestAttributes {
        AnalyticsReportRequestAttributes {
            access_type: None,
            stopped_due_to_inactivity: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AccessType {
    #[serde(rename = "ONE_TIME_SNAPSHOT")]
    OneTimeSnapshot,
    #[serde(rename = "ONGOING")]
    Ongoing,
}

impl Default for AccessType {
    fn default() -> AccessType {
        Self::OneTimeSnapshot
    }
}

