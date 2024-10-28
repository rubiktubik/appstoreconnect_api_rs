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
pub enum CiTestStatus {
    #[serde(rename = "SUCCESS")]
    Success,
    #[serde(rename = "FAILURE")]
    Failure,
    #[serde(rename = "MIXED")]
    Mixed,
    #[serde(rename = "SKIPPED")]
    Skipped,
    #[serde(rename = "EXPECTED_FAILURE")]
    ExpectedFailure,

}

impl std::fmt::Display for CiTestStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Success => write!(f, "SUCCESS"),
            Self::Failure => write!(f, "FAILURE"),
            Self::Mixed => write!(f, "MIXED"),
            Self::Skipped => write!(f, "SKIPPED"),
            Self::ExpectedFailure => write!(f, "EXPECTED_FAILURE"),
        }
    }
}

impl Default for CiTestStatus {
    fn default() -> CiTestStatus {
        Self::Success
    }
}
