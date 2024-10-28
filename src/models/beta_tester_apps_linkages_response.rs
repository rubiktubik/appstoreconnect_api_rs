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
pub struct BetaTesterAppsLinkagesResponse {
    #[serde(rename = "data")]
    pub data: Vec<models::AlternativeDistributionKeyCreateRequestDataRelationshipsAppData>,
    #[serde(rename = "links")]
    pub links: Box<models::PagedDocumentLinks>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<models::PagingInformation>>,
}

impl BetaTesterAppsLinkagesResponse {
    pub fn new(data: Vec<models::AlternativeDistributionKeyCreateRequestDataRelationshipsAppData>, links: models::PagedDocumentLinks) -> BetaTesterAppsLinkagesResponse {
        BetaTesterAppsLinkagesResponse {
            data,
            links: Box::new(links),
            meta: None,
        }
    }
}

