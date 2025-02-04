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
pub struct ProfileCreateRequestDataAttributes {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "profileType")]
    pub profile_type: ProfileType,
}

impl ProfileCreateRequestDataAttributes {
    pub fn new(name: String, profile_type: ProfileType) -> ProfileCreateRequestDataAttributes {
        ProfileCreateRequestDataAttributes {
            name,
            profile_type,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ProfileType {
    #[serde(rename = "IOS_APP_DEVELOPMENT")]
    IosAppDevelopment,
    #[serde(rename = "IOS_APP_STORE")]
    IosAppStore,
    #[serde(rename = "IOS_APP_ADHOC")]
    IosAppAdhoc,
    #[serde(rename = "IOS_APP_INHOUSE")]
    IosAppInhouse,
    #[serde(rename = "MAC_APP_DEVELOPMENT")]
    MacAppDevelopment,
    #[serde(rename = "MAC_APP_STORE")]
    MacAppStore,
    #[serde(rename = "MAC_APP_DIRECT")]
    MacAppDirect,
    #[serde(rename = "TVOS_APP_DEVELOPMENT")]
    TvosAppDevelopment,
    #[serde(rename = "TVOS_APP_STORE")]
    TvosAppStore,
    #[serde(rename = "TVOS_APP_ADHOC")]
    TvosAppAdhoc,
    #[serde(rename = "TVOS_APP_INHOUSE")]
    TvosAppInhouse,
    #[serde(rename = "MAC_CATALYST_APP_DEVELOPMENT")]
    MacCatalystAppDevelopment,
    #[serde(rename = "MAC_CATALYST_APP_STORE")]
    MacCatalystAppStore,
    #[serde(rename = "MAC_CATALYST_APP_DIRECT")]
    MacCatalystAppDirect,
}

impl Default for ProfileType {
    fn default() -> ProfileType {
        Self::IosAppDevelopment
    }
}

