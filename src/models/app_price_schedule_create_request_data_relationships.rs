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
pub struct AppPriceScheduleCreateRequestDataRelationships {
    #[serde(rename = "app")]
    pub app: Box<models::AnalyticsReportRequestCreateRequestDataRelationshipsApp>,
    #[serde(rename = "baseTerritory")]
    pub base_territory: Box<models::AppPriceScheduleCreateRequestDataRelationshipsBaseTerritory>,
    #[serde(rename = "manualPrices")]
    pub manual_prices: Box<models::AppPriceScheduleCreateRequestDataRelationshipsManualPrices>,
}

impl AppPriceScheduleCreateRequestDataRelationships {
    pub fn new(app: models::AnalyticsReportRequestCreateRequestDataRelationshipsApp, base_territory: models::AppPriceScheduleCreateRequestDataRelationshipsBaseTerritory, manual_prices: models::AppPriceScheduleCreateRequestDataRelationshipsManualPrices) -> AppPriceScheduleCreateRequestDataRelationships {
        AppPriceScheduleCreateRequestDataRelationships {
            app: Box::new(app),
            base_territory: Box::new(base_territory),
            manual_prices: Box::new(manual_prices),
        }
    }
}
