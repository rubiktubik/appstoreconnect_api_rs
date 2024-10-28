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
pub struct ScmGitReferenceAttributes {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "canonicalName", skip_serializing_if = "Option::is_none")]
    pub canonical_name: Option<String>,
    #[serde(rename = "isDeleted", skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<models::CiGitRefKind>,
}

impl ScmGitReferenceAttributes {
    pub fn new() -> ScmGitReferenceAttributes {
        ScmGitReferenceAttributes {
            name: None,
            canonical_name: None,
            is_deleted: None,
            kind: None,
        }
    }
}
