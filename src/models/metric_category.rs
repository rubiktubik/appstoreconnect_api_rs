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

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MetricCategory {
    #[serde(rename = "HANG")]
    Hang,
    #[serde(rename = "LAUNCH")]
    Launch,
    #[serde(rename = "MEMORY")]
    Memory,
    #[serde(rename = "DISK")]
    Disk,
    #[serde(rename = "BATTERY")]
    Battery,
    #[serde(rename = "TERMINATION")]
    Termination,
    #[serde(rename = "ANIMATION")]
    Animation,

}

impl std::fmt::Display for MetricCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Hang => write!(f, "HANG"),
            Self::Launch => write!(f, "LAUNCH"),
            Self::Memory => write!(f, "MEMORY"),
            Self::Disk => write!(f, "DISK"),
            Self::Battery => write!(f, "BATTERY"),
            Self::Termination => write!(f, "TERMINATION"),
            Self::Animation => write!(f, "ANIMATION"),
        }
    }
}

impl Default for MetricCategory {
    fn default() -> MetricCategory {
        Self::Hang
    }
}
