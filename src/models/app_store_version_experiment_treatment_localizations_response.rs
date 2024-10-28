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
pub struct AppStoreVersionExperimentTreatmentLocalizationsResponse {
    #[serde(rename = "data")]
    pub data: Vec<models::AppStoreVersionExperimentTreatmentLocalization>,
    #[serde(rename = "included", skip_serializing_if = "Option::is_none")]
    pub included: Option<Vec<models::AppStoreVersionExperimentTreatmentLocalizationsResponseIncludedInner>>,
    #[serde(rename = "links")]
    pub links: Box<models::PagedDocumentLinks>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<models::PagingInformation>>,
}

impl AppStoreVersionExperimentTreatmentLocalizationsResponse {
    pub fn new(data: Vec<models::AppStoreVersionExperimentTreatmentLocalization>, links: models::PagedDocumentLinks) -> AppStoreVersionExperimentTreatmentLocalizationsResponse {
        AppStoreVersionExperimentTreatmentLocalizationsResponse {
            data,
            included: None,
            links: Box::new(links),
            meta: None,
        }
    }
}

