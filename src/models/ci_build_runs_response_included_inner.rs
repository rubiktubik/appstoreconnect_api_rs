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
pub enum CiBuildRunsResponseIncludedInner {
    Build(Box<models::Build>),
    CiWorkflow(Box<models::CiWorkflow>),
    CiProduct(Box<models::CiProduct>),
    ScmGitReference(Box<models::ScmGitReference>),
    ScmPullRequest(Box<models::ScmPullRequest>),
}

impl Default for CiBuildRunsResponseIncludedInner {
    fn default() -> Self {
        Self::Build(Default::default())
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "builds")]
    Builds,
    #[serde(rename = "ciWorkflows")]
    CiWorkflows,
    #[serde(rename = "ciProducts")]
    CiProducts,
    #[serde(rename = "scmGitReferences")]
    ScmGitReferences,
    #[serde(rename = "scmPullRequests")]
    ScmPullRequests,
}

impl Default for Type {
    fn default() -> Type {
        Self::Builds
    }
}
