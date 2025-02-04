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
pub struct InAppPurchaseV2UpdateRequestDataAttributes {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "reviewNote", skip_serializing_if = "Option::is_none")]
    pub review_note: Option<String>,
    #[serde(rename = "familySharable", skip_serializing_if = "Option::is_none")]
    pub family_sharable: Option<bool>,
}

impl InAppPurchaseV2UpdateRequestDataAttributes {
    pub fn new() -> InAppPurchaseV2UpdateRequestDataAttributes {
        InAppPurchaseV2UpdateRequestDataAttributes {
            name: None,
            review_note: None,
            family_sharable: None,
        }
    }
}

