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
pub struct SubscriptionOfferCodeRelationshipsCustomCodes {
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<Box<models::RelationshipLinks>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<models::PagingInformation>>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<models::SubscriptionOfferCodeRelationshipsCustomCodesDataInner>>,
}

impl SubscriptionOfferCodeRelationshipsCustomCodes {
    pub fn new() -> SubscriptionOfferCodeRelationshipsCustomCodes {
        SubscriptionOfferCodeRelationshipsCustomCodes {
            links: None,
            meta: None,
            data: None,
        }
    }
}

