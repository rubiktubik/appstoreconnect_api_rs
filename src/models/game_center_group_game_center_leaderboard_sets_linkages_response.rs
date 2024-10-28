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
pub struct GameCenterGroupGameCenterLeaderboardSetsLinkagesResponse {
    #[serde(rename = "data")]
    pub data: Vec<models::GameCenterDetailRelationshipsGameCenterLeaderboardSetsDataInner>,
    #[serde(rename = "links")]
    pub links: Box<models::PagedDocumentLinks>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<models::PagingInformation>>,
}

impl GameCenterGroupGameCenterLeaderboardSetsLinkagesResponse {
    pub fn new(data: Vec<models::GameCenterDetailRelationshipsGameCenterLeaderboardSetsDataInner>, links: models::PagedDocumentLinks) -> GameCenterGroupGameCenterLeaderboardSetsLinkagesResponse {
        GameCenterGroupGameCenterLeaderboardSetsLinkagesResponse {
            data,
            links: Box::new(links),
            meta: None,
        }
    }
}
