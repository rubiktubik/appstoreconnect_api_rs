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
pub struct PromotedPurchaseCreateRequestDataAttributes {
    #[serde(rename = "visibleForAllUsers")]
    pub visible_for_all_users: bool,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl PromotedPurchaseCreateRequestDataAttributes {
    pub fn new(visible_for_all_users: bool) -> PromotedPurchaseCreateRequestDataAttributes {
        PromotedPurchaseCreateRequestDataAttributes {
            visible_for_all_users,
            enabled: None,
        }
    }
}

