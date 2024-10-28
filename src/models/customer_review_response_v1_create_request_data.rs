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
pub struct CustomerReviewResponseV1CreateRequestData {
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "attributes")]
    pub attributes: Box<models::CustomerReviewResponseV1CreateRequestDataAttributes>,
    #[serde(rename = "relationships")]
    pub relationships: Box<models::CustomerReviewResponseV1CreateRequestDataRelationships>,
}

impl CustomerReviewResponseV1CreateRequestData {
    pub fn new(r#type: Type, attributes: models::CustomerReviewResponseV1CreateRequestDataAttributes, relationships: models::CustomerReviewResponseV1CreateRequestDataRelationships) -> CustomerReviewResponseV1CreateRequestData {
        CustomerReviewResponseV1CreateRequestData {
            r#type,
            attributes: Box::new(attributes),
            relationships: Box::new(relationships),
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "customerReviewResponses")]
    CustomerReviewResponses,
}

impl Default for Type {
    fn default() -> Type {
        Self::CustomerReviewResponses
    }
}
