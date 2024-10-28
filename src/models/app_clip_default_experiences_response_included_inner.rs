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
pub enum AppClipDefaultExperiencesResponseIncludedInner {
    AppClip(Box<models::AppClip>),
    AppStoreVersion(Box<models::AppStoreVersion>),
    AppClipDefaultExperienceLocalization(Box<models::AppClipDefaultExperienceLocalization>),
    AppClipAppStoreReviewDetail(Box<models::AppClipAppStoreReviewDetail>),
}

impl Default for AppClipDefaultExperiencesResponseIncludedInner {
    fn default() -> Self {
        Self::AppClip(Default::default())
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "appClips")]
    AppClips,
    #[serde(rename = "appStoreVersions")]
    AppStoreVersions,
    #[serde(rename = "appClipDefaultExperienceLocalizations")]
    AppClipDefaultExperienceLocalizations,
    #[serde(rename = "appClipAppStoreReviewDetails")]
    AppClipAppStoreReviewDetails,
}

impl Default for Type {
    fn default() -> Type {
        Self::AppClips
    }
}

