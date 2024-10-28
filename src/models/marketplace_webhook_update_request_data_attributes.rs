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
pub struct MarketplaceWebhookUpdateRequestDataAttributes {
    #[serde(rename = "endpointUrl", skip_serializing_if = "Option::is_none")]
    pub endpoint_url: Option<String>,
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}

impl MarketplaceWebhookUpdateRequestDataAttributes {
    pub fn new() -> MarketplaceWebhookUpdateRequestDataAttributes {
        MarketplaceWebhookUpdateRequestDataAttributes {
            endpoint_url: None,
            secret: None,
        }
    }
}
