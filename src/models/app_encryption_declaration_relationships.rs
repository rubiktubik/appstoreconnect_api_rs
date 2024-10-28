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
pub struct AppEncryptionDeclarationRelationships {
    #[serde(rename = "app", skip_serializing_if = "Option::is_none")]
    pub app: Option<Box<models::AppEncryptionDeclarationRelationshipsApp>>,
    #[serde(rename = "builds", skip_serializing_if = "Option::is_none")]
    pub builds: Option<Box<models::AppEncryptionDeclarationRelationshipsBuilds>>,
    #[serde(rename = "appEncryptionDeclarationDocument", skip_serializing_if = "Option::is_none")]
    pub app_encryption_declaration_document: Option<Box<models::AppEncryptionDeclarationRelationshipsAppEncryptionDeclarationDocument>>,
}

impl AppEncryptionDeclarationRelationships {
    pub fn new() -> AppEncryptionDeclarationRelationships {
        AppEncryptionDeclarationRelationships {
            app: None,
            builds: None,
            app_encryption_declaration_document: None,
        }
    }
}
