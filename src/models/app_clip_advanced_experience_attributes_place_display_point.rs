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
pub struct AppClipAdvancedExperienceAttributesPlaceDisplayPoint {
    #[serde(rename = "coordinates", skip_serializing_if = "Option::is_none")]
    pub coordinates: Option<Box<models::AppClipAdvancedExperienceAttributesPlaceDisplayPointCoordinates>>,
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
}

impl AppClipAdvancedExperienceAttributesPlaceDisplayPoint {
    pub fn new() -> AppClipAdvancedExperienceAttributesPlaceDisplayPoint {
        AppClipAdvancedExperienceAttributesPlaceDisplayPoint {
            coordinates: None,
            source: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Source {
    #[serde(rename = "CALCULATED")]
    Calculated,
    #[serde(rename = "MANUALLY_PLACED")]
    ManuallyPlaced,
}

impl Default for Source {
    fn default() -> Source {
        Self::Calculated
    }
}

