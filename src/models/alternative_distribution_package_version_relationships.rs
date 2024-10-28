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
pub struct AlternativeDistributionPackageVersionRelationships {
    #[serde(rename = "variants", skip_serializing_if = "Option::is_none")]
    pub variants: Option<Box<models::AlternativeDistributionPackageVersionRelationshipsVariants>>,
    #[serde(rename = "deltas", skip_serializing_if = "Option::is_none")]
    pub deltas: Option<Box<models::AlternativeDistributionPackageVersionRelationshipsDeltas>>,
    #[serde(rename = "alternativeDistributionPackage", skip_serializing_if = "Option::is_none")]
    pub alternative_distribution_package: Option<Box<models::AlternativeDistributionPackageVersionRelationshipsAlternativeDistributionPackage>>,
}

impl AlternativeDistributionPackageVersionRelationships {
    pub fn new() -> AlternativeDistributionPackageVersionRelationships {
        AlternativeDistributionPackageVersionRelationships {
            variants: None,
            deltas: None,
            alternative_distribution_package: None,
        }
    }
}
