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
pub struct PagingInformationPaging {
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
    #[serde(rename = "limit")]
    pub limit: i32,
}

impl PagingInformationPaging {
    pub fn new(limit: i32) -> PagingInformationPaging {
        PagingInformationPaging {
            total: None,
            limit,
        }
    }
}

