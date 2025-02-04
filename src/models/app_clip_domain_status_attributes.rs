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
pub struct AppClipDomainStatusAttributes {
    #[serde(rename = "domains", skip_serializing_if = "Option::is_none")]
    pub domains: Option<Vec<models::AppClipDomainStatusAttributesDomainsInner>>,
    #[serde(rename = "lastUpdatedDate", skip_serializing_if = "Option::is_none")]
    pub last_updated_date: Option<String>,
}

impl AppClipDomainStatusAttributes {
    pub fn new() -> AppClipDomainStatusAttributes {
        AppClipDomainStatusAttributes {
            domains: None,
            last_updated_date: None,
        }
    }
}

