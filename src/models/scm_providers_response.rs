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
pub struct ScmProvidersResponse {
    #[serde(rename = "data")]
    pub data: Vec<models::ScmProvider>,
    #[serde(rename = "links")]
    pub links: Box<models::PagedDocumentLinks>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<models::PagingInformation>>,
}

impl ScmProvidersResponse {
    pub fn new(data: Vec<models::ScmProvider>, links: models::PagedDocumentLinks) -> ScmProvidersResponse {
        ScmProvidersResponse {
            data,
            links: Box::new(links),
            meta: None,
        }
    }
}

