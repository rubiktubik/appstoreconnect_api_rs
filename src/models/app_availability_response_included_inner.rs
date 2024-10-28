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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AppAvailabilityResponseIncludedInner {
    App(Box<models::App>),
    Territory(Box<models::Territory>),
}

impl Default for AppAvailabilityResponseIncludedInner {
    fn default() -> Self {
        Self::App(Default::default())
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "apps")]
    Apps,
    #[serde(rename = "territories")]
    Territories,
}

impl Default for Type {
    fn default() -> Type {
        Self::Apps
    }
}

