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
pub struct AgeRatingDeclarationResponse {
    #[serde(rename = "data")]
    pub data: Box<models::AgeRatingDeclaration>,
    #[serde(rename = "links")]
    pub links: Box<models::DocumentLinks>,
}

impl AgeRatingDeclarationResponse {
    pub fn new(data: models::AgeRatingDeclaration, links: models::DocumentLinks) -> AgeRatingDeclarationResponse {
        AgeRatingDeclarationResponse {
            data: Box::new(data),
            links: Box::new(links),
        }
    }
}

