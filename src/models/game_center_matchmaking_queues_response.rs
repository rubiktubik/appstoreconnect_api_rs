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
pub struct GameCenterMatchmakingQueuesResponse {
    #[serde(rename = "data")]
    pub data: Vec<models::GameCenterMatchmakingQueue>,
    #[serde(rename = "included", skip_serializing_if = "Option::is_none")]
    pub included: Option<Vec<models::GameCenterMatchmakingRuleSet>>,
    #[serde(rename = "links")]
    pub links: Box<models::PagedDocumentLinks>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<models::PagingInformation>>,
}

impl GameCenterMatchmakingQueuesResponse {
    pub fn new(data: Vec<models::GameCenterMatchmakingQueue>, links: models::PagedDocumentLinks) -> GameCenterMatchmakingQueuesResponse {
        GameCenterMatchmakingQueuesResponse {
            data,
            included: None,
            links: Box::new(links),
            meta: None,
        }
    }
}

