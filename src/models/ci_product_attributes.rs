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
pub struct CiProductAttributes {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "createdDate", skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    #[serde(rename = "productType", skip_serializing_if = "Option::is_none")]
    pub product_type: Option<ProductType>,
}

impl CiProductAttributes {
    pub fn new() -> CiProductAttributes {
        CiProductAttributes {
            name: None,
            created_date: None,
            product_type: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ProductType {
    #[serde(rename = "APP")]
    App,
    #[serde(rename = "FRAMEWORK")]
    Framework,
}

impl Default for ProductType {
    fn default() -> ProductType {
        Self::App
    }
}

