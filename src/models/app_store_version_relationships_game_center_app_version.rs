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
pub struct AppStoreVersionRelationshipsGameCenterAppVersion {
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<Box<models::RelationshipLinks>>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Box<models::AppStoreVersionRelationshipsGameCenterAppVersionData>>,
}

impl AppStoreVersionRelationshipsGameCenterAppVersion {
    pub fn new() -> AppStoreVersionRelationshipsGameCenterAppVersion {
        AppStoreVersionRelationshipsGameCenterAppVersion {
            links: None,
            data: None,
        }
    }
}

