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
pub struct AppEncryptionDeclarationDocumentCreateRequest {
    #[serde(rename = "data")]
    pub data: Box<models::AppEncryptionDeclarationDocumentCreateRequestData>,
}

impl AppEncryptionDeclarationDocumentCreateRequest {
    pub fn new(data: models::AppEncryptionDeclarationDocumentCreateRequestData) -> AppEncryptionDeclarationDocumentCreateRequest {
        AppEncryptionDeclarationDocumentCreateRequest {
            data: Box::new(data),
        }
    }
}

