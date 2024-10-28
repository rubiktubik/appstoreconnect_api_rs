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
pub struct AppAvailabilityV2CreateRequestDataRelationships {
    #[serde(rename = "app")]
    pub app: Box<models::AnalyticsReportRequestCreateRequestDataRelationshipsApp>,
    #[serde(rename = "territoryAvailabilities")]
    pub territory_availabilities: Box<models::AppAvailabilityV2CreateRequestDataRelationshipsTerritoryAvailabilities>,
}

impl AppAvailabilityV2CreateRequestDataRelationships {
    pub fn new(app: models::AnalyticsReportRequestCreateRequestDataRelationshipsApp, territory_availabilities: models::AppAvailabilityV2CreateRequestDataRelationshipsTerritoryAvailabilities) -> AppAvailabilityV2CreateRequestDataRelationships {
        AppAvailabilityV2CreateRequestDataRelationships {
            app: Box::new(app),
            territory_availabilities: Box::new(territory_availabilities),
        }
    }
}
