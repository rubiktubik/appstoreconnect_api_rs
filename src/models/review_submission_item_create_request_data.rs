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
pub struct ReviewSubmissionItemCreateRequestData {
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "relationships")]
    pub relationships: Box<models::ReviewSubmissionItemCreateRequestDataRelationships>,
}

impl ReviewSubmissionItemCreateRequestData {
    pub fn new(r#type: Type, relationships: models::ReviewSubmissionItemCreateRequestDataRelationships) -> ReviewSubmissionItemCreateRequestData {
        ReviewSubmissionItemCreateRequestData {
            r#type,
            relationships: Box::new(relationships),
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "reviewSubmissionItems")]
    ReviewSubmissionItems,
}

impl Default for Type {
    fn default() -> Type {
        Self::ReviewSubmissionItems
    }
}

