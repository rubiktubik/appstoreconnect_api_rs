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
pub struct AlternativeDistributionPackageVariant {
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Box<models::AlternativeDistributionPackageDeltaAttributes>>,
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<Box<models::ResourceLinks>>,
}

impl AlternativeDistributionPackageVariant {
    pub fn new(r#type: Type, id: String) -> AlternativeDistributionPackageVariant {
        AlternativeDistributionPackageVariant {
            r#type,
            id,
            attributes: None,
            links: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "alternativeDistributionPackageVariants")]
    AlternativeDistributionPackageVariants,
}

impl Default for Type {
    fn default() -> Type {
        Self::AlternativeDistributionPackageVariants
    }
}

