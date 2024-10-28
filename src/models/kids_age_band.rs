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
pub enum KidsAgeBand {
    #[serde(rename = "FIVE_AND_UNDER")]
    FiveAndUnder,
    #[serde(rename = "SIX_TO_EIGHT")]
    SixToEight,
    #[serde(rename = "NINE_TO_ELEVEN")]
    NineToEleven,

}

impl std::fmt::Display for KidsAgeBand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FiveAndUnder => write!(f, "FIVE_AND_UNDER"),
            Self::SixToEight => write!(f, "SIX_TO_EIGHT"),
            Self::NineToEleven => write!(f, "NINE_TO_ELEVEN"),
        }
    }
}

impl Default for KidsAgeBand {
    fn default() -> KidsAgeBand {
        Self::FiveAndUnder
    }
}
