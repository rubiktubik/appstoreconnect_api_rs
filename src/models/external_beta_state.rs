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
pub enum ExternalBetaState {
    #[serde(rename = "PROCESSING")]
    Processing,
    #[serde(rename = "PROCESSING_EXCEPTION")]
    ProcessingException,
    #[serde(rename = "MISSING_EXPORT_COMPLIANCE")]
    MissingExportCompliance,
    #[serde(rename = "READY_FOR_BETA_TESTING")]
    ReadyForBetaTesting,
    #[serde(rename = "IN_BETA_TESTING")]
    InBetaTesting,
    #[serde(rename = "EXPIRED")]
    Expired,
    #[serde(rename = "READY_FOR_BETA_SUBMISSION")]
    ReadyForBetaSubmission,
    #[serde(rename = "IN_EXPORT_COMPLIANCE_REVIEW")]
    InExportComplianceReview,
    #[serde(rename = "WAITING_FOR_BETA_REVIEW")]
    WaitingForBetaReview,
    #[serde(rename = "IN_BETA_REVIEW")]
    InBetaReview,
    #[serde(rename = "BETA_REJECTED")]
    BetaRejected,
    #[serde(rename = "BETA_APPROVED")]
    BetaApproved,

}

impl std::fmt::Display for ExternalBetaState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Processing => write!(f, "PROCESSING"),
            Self::ProcessingException => write!(f, "PROCESSING_EXCEPTION"),
            Self::MissingExportCompliance => write!(f, "MISSING_EXPORT_COMPLIANCE"),
            Self::ReadyForBetaTesting => write!(f, "READY_FOR_BETA_TESTING"),
            Self::InBetaTesting => write!(f, "IN_BETA_TESTING"),
            Self::Expired => write!(f, "EXPIRED"),
            Self::ReadyForBetaSubmission => write!(f, "READY_FOR_BETA_SUBMISSION"),
            Self::InExportComplianceReview => write!(f, "IN_EXPORT_COMPLIANCE_REVIEW"),
            Self::WaitingForBetaReview => write!(f, "WAITING_FOR_BETA_REVIEW"),
            Self::InBetaReview => write!(f, "IN_BETA_REVIEW"),
            Self::BetaRejected => write!(f, "BETA_REJECTED"),
            Self::BetaApproved => write!(f, "BETA_APPROVED"),
        }
    }
}

impl Default for ExternalBetaState {
    fn default() -> ExternalBetaState {
        Self::Processing
    }
}

