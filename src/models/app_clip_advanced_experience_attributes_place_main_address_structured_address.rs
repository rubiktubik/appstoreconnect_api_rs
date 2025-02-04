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
pub struct AppClipAdvancedExperienceAttributesPlaceMainAddressStructuredAddress {
    #[serde(rename = "streetAddress", skip_serializing_if = "Option::is_none")]
    pub street_address: Option<Vec<String>>,
    #[serde(rename = "floor", skip_serializing_if = "Option::is_none")]
    pub floor: Option<String>,
    #[serde(rename = "neighborhood", skip_serializing_if = "Option::is_none")]
    pub neighborhood: Option<String>,
    #[serde(rename = "locality", skip_serializing_if = "Option::is_none")]
    pub locality: Option<String>,
    #[serde(rename = "stateProvince", skip_serializing_if = "Option::is_none")]
    pub state_province: Option<String>,
    #[serde(rename = "postalCode", skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[serde(rename = "countryCode", skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
}

impl AppClipAdvancedExperienceAttributesPlaceMainAddressStructuredAddress {
    pub fn new() -> AppClipAdvancedExperienceAttributesPlaceMainAddressStructuredAddress {
        AppClipAdvancedExperienceAttributesPlaceMainAddressStructuredAddress {
            street_address: None,
            floor: None,
            neighborhood: None,
            locality: None,
            state_province: None,
            postal_code: None,
            country_code: None,
        }
    }
}

