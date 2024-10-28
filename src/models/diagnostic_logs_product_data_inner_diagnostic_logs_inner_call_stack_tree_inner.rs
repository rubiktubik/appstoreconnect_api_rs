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
pub struct DiagnosticLogsProductDataInnerDiagnosticLogsInnerCallStackTreeInner {
    #[serde(rename = "callStackPerThread", skip_serializing_if = "Option::is_none")]
    pub call_stack_per_thread: Option<bool>,
    #[serde(rename = "callStacks", skip_serializing_if = "Option::is_none")]
    pub call_stacks: Option<Vec<models::DiagnosticLogsProductDataInnerDiagnosticLogsInnerCallStackTreeInnerCallStacksInner>>,
}

impl DiagnosticLogsProductDataInnerDiagnosticLogsInnerCallStackTreeInner {
    pub fn new() -> DiagnosticLogsProductDataInnerDiagnosticLogsInnerCallStackTreeInner {
        DiagnosticLogsProductDataInnerDiagnosticLogsInnerCallStackTreeInner {
            call_stack_per_thread: None,
            call_stacks: None,
        }
    }
}
