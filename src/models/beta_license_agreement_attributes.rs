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
pub struct BetaLicenseAgreementAttributes {
    #[serde(rename = "agreementText", skip_serializing_if = "Option::is_none")]
    pub agreement_text: Option<String>,
}

impl BetaLicenseAgreementAttributes {
    pub fn new() -> BetaLicenseAgreementAttributes {
        BetaLicenseAgreementAttributes {
            agreement_text: None,
        }
    }
}

