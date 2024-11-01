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
pub enum BuildsResponseIncludedInner {
    PrereleaseVersion(Box<models::PrereleaseVersion>),
    BetaTester(Box<models::BetaTester>),
    BetaGroup(Box<models::BetaGroup>),
    BetaBuildLocalization(Box<models::BetaBuildLocalization>),
    AppEncryptionDeclaration(Box<models::AppEncryptionDeclaration>),
    BetaAppReviewSubmission(Box<models::BetaAppReviewSubmission>),
    App(Box<models::App>),
    BuildBetaDetail(Box<models::BuildBetaDetail>),
    AppStoreVersion(Box<models::AppStoreVersion>),
    BuildIcon(Box<models::BuildIcon>),
    BuildBundle(Box<models::BuildBundle>),
}

impl Default for BuildsResponseIncludedInner {
    fn default() -> Self {
        Self::PrereleaseVersion(Default::default())
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "preReleaseVersions")]
    PreReleaseVersions,
    #[serde(rename = "betaTesters")]
    BetaTesters,
    #[serde(rename = "betaGroups")]
    BetaGroups,
    #[serde(rename = "betaBuildLocalizations")]
    BetaBuildLocalizations,
    #[serde(rename = "appEncryptionDeclarations")]
    AppEncryptionDeclarations,
    #[serde(rename = "betaAppReviewSubmissions")]
    BetaAppReviewSubmissions,
    #[serde(rename = "apps")]
    Apps,
    #[serde(rename = "buildBetaDetails")]
    BuildBetaDetails,
    #[serde(rename = "appStoreVersions")]
    AppStoreVersions,
    #[serde(rename = "buildIcons")]
    BuildIcons,
    #[serde(rename = "buildBundles")]
    BuildBundles,
}

impl Default for Type {
    fn default() -> Type {
        Self::PreReleaseVersions
    }
}

