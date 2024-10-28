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
pub struct CiWorkflowRelationshipsMacOsVersionData {
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "id")]
    pub id: String,
}

impl CiWorkflowRelationshipsMacOsVersionData {
    pub fn new(r#type: Type, id: String) -> CiWorkflowRelationshipsMacOsVersionData {
        CiWorkflowRelationshipsMacOsVersionData {
            r#type,
            id,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "ciMacOsVersions")]
    CiMacOsVersions,
}

impl Default for Type {
    fn default() -> Type {
        Self::CiMacOsVersions
    }
}

