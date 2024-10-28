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
pub struct AppEventScreenshotCreateRequestDataRelationshipsAppEventLocalization {
    #[serde(rename = "data")]
    pub data: Box<models::AppEventScreenshotRelationshipsAppEventLocalizationData>,
}

impl AppEventScreenshotCreateRequestDataRelationshipsAppEventLocalization {
    pub fn new(data: models::AppEventScreenshotRelationshipsAppEventLocalizationData) -> AppEventScreenshotCreateRequestDataRelationshipsAppEventLocalization {
        AppEventScreenshotCreateRequestDataRelationshipsAppEventLocalization {
            data: Box::new(data),
        }
    }
}
