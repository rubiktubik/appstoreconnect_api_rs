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
pub struct ScmProviderResponse {
    #[serde(rename = "data")]
    pub data: Box<models::ScmProvider>,
    #[serde(rename = "links")]
    pub links: Box<models::DocumentLinks>,
}

impl ScmProviderResponse {
    pub fn new(data: models::ScmProvider, links: models::DocumentLinks) -> ScmProviderResponse {
        ScmProviderResponse {
            data: Box::new(data),
            links: Box::new(links),
        }
    }
}

