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
pub struct AppInfoLocalizationCreateRequestDataRelationships {
    #[serde(rename = "appInfo")]
    pub app_info: Box<models::AppInfoLocalizationCreateRequestDataRelationshipsAppInfo>,
}

impl AppInfoLocalizationCreateRequestDataRelationships {
    pub fn new(app_info: models::AppInfoLocalizationCreateRequestDataRelationshipsAppInfo) -> AppInfoLocalizationCreateRequestDataRelationships {
        AppInfoLocalizationCreateRequestDataRelationships {
            app_info: Box::new(app_info),
        }
    }
}
