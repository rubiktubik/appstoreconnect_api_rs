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
pub struct AppPreviewCreateRequestDataRelationships {
    #[serde(rename = "appPreviewSet")]
    pub app_preview_set: Box<models::AppPreviewCreateRequestDataRelationshipsAppPreviewSet>,
}

impl AppPreviewCreateRequestDataRelationships {
    pub fn new(app_preview_set: models::AppPreviewCreateRequestDataRelationshipsAppPreviewSet) -> AppPreviewCreateRequestDataRelationships {
        AppPreviewCreateRequestDataRelationships {
            app_preview_set: Box::new(app_preview_set),
        }
    }
}
