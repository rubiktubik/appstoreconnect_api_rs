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
pub struct AppEventRelationships {
    #[serde(rename = "localizations", skip_serializing_if = "Option::is_none")]
    pub localizations: Option<Box<models::AppEventRelationshipsLocalizations>>,
}

impl AppEventRelationships {
    pub fn new() -> AppEventRelationships {
        AppEventRelationships {
            localizations: None,
        }
    }
}

