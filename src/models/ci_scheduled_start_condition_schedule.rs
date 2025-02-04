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
pub struct CiScheduledStartConditionSchedule {
    #[serde(rename = "frequency", skip_serializing_if = "Option::is_none")]
    pub frequency: Option<Frequency>,
    #[serde(rename = "days", skip_serializing_if = "Option::is_none")]
    pub days: Option<Vec<Days>>,
    #[serde(rename = "hour", skip_serializing_if = "Option::is_none")]
    pub hour: Option<i32>,
    #[serde(rename = "minute", skip_serializing_if = "Option::is_none")]
    pub minute: Option<i32>,
    #[serde(rename = "timezone", skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

impl CiScheduledStartConditionSchedule {
    pub fn new() -> CiScheduledStartConditionSchedule {
        CiScheduledStartConditionSchedule {
            frequency: None,
            days: None,
            hour: None,
            minute: None,
            timezone: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Frequency {
    #[serde(rename = "WEEKLY")]
    Weekly,
    #[serde(rename = "DAILY")]
    Daily,
    #[serde(rename = "HOURLY")]
    Hourly,
}

impl Default for Frequency {
    fn default() -> Frequency {
        Self::Weekly
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Days {
    #[serde(rename = "SUNDAY")]
    Sunday,
    #[serde(rename = "MONDAY")]
    Monday,
    #[serde(rename = "TUESDAY")]
    Tuesday,
    #[serde(rename = "WEDNESDAY")]
    Wednesday,
    #[serde(rename = "THURSDAY")]
    Thursday,
    #[serde(rename = "FRIDAY")]
    Friday,
    #[serde(rename = "SATURDAY")]
    Saturday,
}

impl Default for Days {
    fn default() -> Days {
        Self::Sunday
    }
}

