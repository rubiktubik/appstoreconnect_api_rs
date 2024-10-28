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
pub struct BetaLicenseAgreementWithoutIncludesResponse {
    #[serde(rename = "data")]
    pub data: Box<models::BetaLicenseAgreement>,
    #[serde(rename = "links")]
    pub links: Box<models::DocumentLinks>,
}

impl BetaLicenseAgreementWithoutIncludesResponse {
    pub fn new(data: models::BetaLicenseAgreement, links: models::DocumentLinks) -> BetaLicenseAgreementWithoutIncludesResponse {
        BetaLicenseAgreementWithoutIncludesResponse {
            data: Box::new(data),
            links: Box::new(links),
        }
    }
}

