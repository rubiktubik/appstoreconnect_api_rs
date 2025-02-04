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
pub struct AlternativeDistributionDomainCreateRequestDataAttributes {
    #[serde(rename = "domain")]
    pub domain: String,
    #[serde(rename = "referenceName")]
    pub reference_name: String,
}

impl AlternativeDistributionDomainCreateRequestDataAttributes {
    pub fn new(domain: String, reference_name: String) -> AlternativeDistributionDomainCreateRequestDataAttributes {
        AlternativeDistributionDomainCreateRequestDataAttributes {
            domain,
            reference_name,
        }
    }
}

