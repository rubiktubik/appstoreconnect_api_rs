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
pub enum CiCompletionStatus {
    #[serde(rename = "SUCCEEDED")]
    Succeeded,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "ERRORED")]
    Errored,
    #[serde(rename = "CANCELED")]
    Canceled,
    #[serde(rename = "SKIPPED")]
    Skipped,

}

impl std::fmt::Display for CiCompletionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Succeeded => write!(f, "SUCCEEDED"),
            Self::Failed => write!(f, "FAILED"),
            Self::Errored => write!(f, "ERRORED"),
            Self::Canceled => write!(f, "CANCELED"),
            Self::Skipped => write!(f, "SKIPPED"),
        }
    }
}

impl Default for CiCompletionStatus {
    fn default() -> CiCompletionStatus {
        Self::Succeeded
    }
}

