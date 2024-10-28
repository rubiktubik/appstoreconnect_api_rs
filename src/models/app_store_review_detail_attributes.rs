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
pub struct AppStoreReviewDetailAttributes {
    #[serde(rename = "contactFirstName", skip_serializing_if = "Option::is_none")]
    pub contact_first_name: Option<String>,
    #[serde(rename = "contactLastName", skip_serializing_if = "Option::is_none")]
    pub contact_last_name: Option<String>,
    #[serde(rename = "contactPhone", skip_serializing_if = "Option::is_none")]
    pub contact_phone: Option<String>,
    #[serde(rename = "contactEmail", skip_serializing_if = "Option::is_none")]
    pub contact_email: Option<String>,
    #[serde(rename = "demoAccountName", skip_serializing_if = "Option::is_none")]
    pub demo_account_name: Option<String>,
    #[serde(rename = "demoAccountPassword", skip_serializing_if = "Option::is_none")]
    pub demo_account_password: Option<String>,
    #[serde(rename = "demoAccountRequired", skip_serializing_if = "Option::is_none")]
    pub demo_account_required: Option<bool>,
    #[serde(rename = "notes", skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl AppStoreReviewDetailAttributes {
    pub fn new() -> AppStoreReviewDetailAttributes {
        AppStoreReviewDetailAttributes {
            contact_first_name: None,
            contact_last_name: None,
            contact_phone: None,
            contact_email: None,
            demo_account_name: None,
            demo_account_password: None,
            demo_account_required: None,
            notes: None,
        }
    }
}

