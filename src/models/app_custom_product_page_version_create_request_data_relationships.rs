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
pub struct AppCustomProductPageVersionCreateRequestDataRelationships {
    #[serde(rename = "appCustomProductPage")]
    pub app_custom_product_page: Box<models::AppCustomProductPageVersionCreateRequestDataRelationshipsAppCustomProductPage>,
    #[serde(rename = "appCustomProductPageLocalizations", skip_serializing_if = "Option::is_none")]
    pub app_custom_product_page_localizations: Option<Box<models::AppCustomProductPageVersionInlineCreateRelationshipsAppCustomProductPageLocalizations>>,
}

impl AppCustomProductPageVersionCreateRequestDataRelationships {
    pub fn new(app_custom_product_page: models::AppCustomProductPageVersionCreateRequestDataRelationshipsAppCustomProductPage) -> AppCustomProductPageVersionCreateRequestDataRelationships {
        AppCustomProductPageVersionCreateRequestDataRelationships {
            app_custom_product_page: Box::new(app_custom_product_page),
            app_custom_product_page_localizations: None,
        }
    }
}
