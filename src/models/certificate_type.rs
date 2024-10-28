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

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CertificateType {
    #[serde(rename = "IOS_DEVELOPMENT")]
    IosDevelopment,
    #[serde(rename = "IOS_DISTRIBUTION")]
    IosDistribution,
    #[serde(rename = "MAC_APP_DISTRIBUTION")]
    MacAppDistribution,
    #[serde(rename = "MAC_INSTALLER_DISTRIBUTION")]
    MacInstallerDistribution,
    #[serde(rename = "MAC_APP_DEVELOPMENT")]
    MacAppDevelopment,
    #[serde(rename = "DEVELOPER_ID_KEXT")]
    DeveloperIdKext,
    #[serde(rename = "DEVELOPER_ID_APPLICATION")]
    DeveloperIdApplication,
    #[serde(rename = "DEVELOPMENT")]
    Development,
    #[serde(rename = "DISTRIBUTION")]
    Distribution,
    #[serde(rename = "PASS_TYPE_ID")]
    PassTypeId,
    #[serde(rename = "PASS_TYPE_ID_WITH_NFC")]
    PassTypeIdWithNfc,

}

impl std::fmt::Display for CertificateType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::IosDevelopment => write!(f, "IOS_DEVELOPMENT"),
            Self::IosDistribution => write!(f, "IOS_DISTRIBUTION"),
            Self::MacAppDistribution => write!(f, "MAC_APP_DISTRIBUTION"),
            Self::MacInstallerDistribution => write!(f, "MAC_INSTALLER_DISTRIBUTION"),
            Self::MacAppDevelopment => write!(f, "MAC_APP_DEVELOPMENT"),
            Self::DeveloperIdKext => write!(f, "DEVELOPER_ID_KEXT"),
            Self::DeveloperIdApplication => write!(f, "DEVELOPER_ID_APPLICATION"),
            Self::Development => write!(f, "DEVELOPMENT"),
            Self::Distribution => write!(f, "DISTRIBUTION"),
            Self::PassTypeId => write!(f, "PASS_TYPE_ID"),
            Self::PassTypeIdWithNfc => write!(f, "PASS_TYPE_ID_WITH_NFC"),
        }
    }
}

impl Default for CertificateType {
    fn default() -> CertificateType {
        Self::IosDevelopment
    }
}

