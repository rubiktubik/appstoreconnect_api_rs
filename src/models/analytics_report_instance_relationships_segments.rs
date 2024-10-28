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
pub struct AnalyticsReportInstanceRelationshipsSegments {
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<Box<models::RelationshipLinks>>,
}

impl AnalyticsReportInstanceRelationshipsSegments {
    pub fn new() -> AnalyticsReportInstanceRelationshipsSegments {
        AnalyticsReportInstanceRelationshipsSegments {
            links: None,
        }
    }
}

