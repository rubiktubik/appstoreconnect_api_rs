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
pub struct DiagnosticSignatureRelationships {
    #[serde(rename = "logs", skip_serializing_if = "Option::is_none")]
    pub logs: Option<Box<models::AnalyticsReportInstanceRelationshipsSegments>>,
}

impl DiagnosticSignatureRelationships {
    pub fn new() -> DiagnosticSignatureRelationships {
        DiagnosticSignatureRelationships {
            logs: None,
        }
    }
}

