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
pub struct CertificateCreateRequestData {
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "attributes")]
    pub attributes: Box<models::CertificateCreateRequestDataAttributes>,
}

impl CertificateCreateRequestData {
    pub fn new(r#type: Type, attributes: models::CertificateCreateRequestDataAttributes) -> CertificateCreateRequestData {
        CertificateCreateRequestData {
            r#type,
            attributes: Box::new(attributes),
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "certificates")]
    Certificates,
}

impl Default for Type {
    fn default() -> Type {
        Self::Certificates
    }
}
