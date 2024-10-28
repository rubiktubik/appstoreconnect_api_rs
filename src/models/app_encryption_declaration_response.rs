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
pub struct AppEncryptionDeclarationResponse {
    #[serde(rename = "data")]
    pub data: Box<models::AppEncryptionDeclaration>,
    #[serde(rename = "included", skip_serializing_if = "Option::is_none")]
    pub included: Option<Vec<models::AppEncryptionDeclarationsResponseIncludedInner>>,
    #[serde(rename = "links")]
    pub links: Box<models::DocumentLinks>,
}

impl AppEncryptionDeclarationResponse {
    pub fn new(data: models::AppEncryptionDeclaration, links: models::DocumentLinks) -> AppEncryptionDeclarationResponse {
        AppEncryptionDeclarationResponse {
            data: Box::new(data),
            included: None,
            links: Box::new(links),
        }
    }
}
