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
pub struct EndUserLicenseAgreementCreateRequestDataRelationships {
    #[serde(rename = "app")]
    pub app: Box<models::AnalyticsReportRequestCreateRequestDataRelationshipsApp>,
    #[serde(rename = "territories")]
    pub territories: Box<models::AppAvailabilityCreateRequestDataRelationshipsAvailableTerritories>,
}

impl EndUserLicenseAgreementCreateRequestDataRelationships {
    pub fn new(app: models::AnalyticsReportRequestCreateRequestDataRelationshipsApp, territories: models::AppAvailabilityCreateRequestDataRelationshipsAvailableTerritories) -> EndUserLicenseAgreementCreateRequestDataRelationships {
        EndUserLicenseAgreementCreateRequestDataRelationships {
            app: Box::new(app),
            territories: Box::new(territories),
        }
    }
}

