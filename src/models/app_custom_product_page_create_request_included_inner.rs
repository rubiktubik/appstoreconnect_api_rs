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
pub enum AppCustomProductPageCreateRequestIncludedInner {
    AppCustomProductPageLocalizationInlineCreate(Box<models::AppCustomProductPageLocalizationInlineCreate>),
    AppCustomProductPageVersionInlineCreate(Box<models::AppCustomProductPageVersionInlineCreate>),
}

impl Default for AppCustomProductPageCreateRequestIncludedInner {
    fn default() -> Self {
        Self::AppCustomProductPageLocalizationInlineCreate(Default::default())
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "appCustomProductPageLocalizations")]
    AppCustomProductPageLocalizations,
    #[serde(rename = "appCustomProductPageVersions")]
    AppCustomProductPageVersions,
}

impl Default for Type {
    fn default() -> Type {
        Self::AppCustomProductPageLocalizations
    }
}

