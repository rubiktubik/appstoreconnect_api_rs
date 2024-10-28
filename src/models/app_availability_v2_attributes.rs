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
pub struct AppAvailabilityV2Attributes {
    #[serde(rename = "availableInNewTerritories", skip_serializing_if = "Option::is_none")]
    pub available_in_new_territories: Option<bool>,
}

impl AppAvailabilityV2Attributes {
    pub fn new() -> AppAvailabilityV2Attributes {
        AppAvailabilityV2Attributes {
            available_in_new_territories: None,
        }
    }
}

