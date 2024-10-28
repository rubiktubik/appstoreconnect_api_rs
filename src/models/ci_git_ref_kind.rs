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
pub enum CiGitRefKind {
    #[serde(rename = "BRANCH")]
    Branch,
    #[serde(rename = "TAG")]
    Tag,

}

impl std::fmt::Display for CiGitRefKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Branch => write!(f, "BRANCH"),
            Self::Tag => write!(f, "TAG"),
        }
    }
}

impl Default for CiGitRefKind {
    fn default() -> CiGitRefKind {
        Self::Branch
    }
}
