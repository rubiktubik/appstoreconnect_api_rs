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
pub struct AppUpdateRequestDataAttributes {
    #[serde(rename = "bundleId", skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    #[serde(rename = "primaryLocale", skip_serializing_if = "Option::is_none")]
    pub primary_locale: Option<String>,
    #[serde(rename = "subscriptionStatusUrl", skip_serializing_if = "Option::is_none")]
    pub subscription_status_url: Option<String>,
    #[serde(rename = "subscriptionStatusUrlVersion", skip_serializing_if = "Option::is_none")]
    pub subscription_status_url_version: Option<models::SubscriptionStatusUrlVersion>,
    #[serde(rename = "subscriptionStatusUrlForSandbox", skip_serializing_if = "Option::is_none")]
    pub subscription_status_url_for_sandbox: Option<String>,
    #[serde(rename = "subscriptionStatusUrlVersionForSandbox", skip_serializing_if = "Option::is_none")]
    pub subscription_status_url_version_for_sandbox: Option<models::SubscriptionStatusUrlVersion>,
    #[serde(rename = "contentRightsDeclaration", skip_serializing_if = "Option::is_none")]
    pub content_rights_declaration: Option<ContentRightsDeclaration>,
    #[serde(rename = "streamlinedPurchasingEnabled", skip_serializing_if = "Option::is_none")]
    pub streamlined_purchasing_enabled: Option<bool>,
}

impl AppUpdateRequestDataAttributes {
    pub fn new() -> AppUpdateRequestDataAttributes {
        AppUpdateRequestDataAttributes {
            bundle_id: None,
            primary_locale: None,
            subscription_status_url: None,
            subscription_status_url_version: None,
            subscription_status_url_for_sandbox: None,
            subscription_status_url_version_for_sandbox: None,
            content_rights_declaration: None,
            streamlined_purchasing_enabled: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ContentRightsDeclaration {
    #[serde(rename = "DOES_NOT_USE_THIRD_PARTY_CONTENT")]
    DoesNotUseThirdPartyContent,
    #[serde(rename = "USES_THIRD_PARTY_CONTENT")]
    UsesThirdPartyContent,
}

impl Default for ContentRightsDeclaration {
    fn default() -> ContentRightsDeclaration {
        Self::DoesNotUseThirdPartyContent
    }
}

