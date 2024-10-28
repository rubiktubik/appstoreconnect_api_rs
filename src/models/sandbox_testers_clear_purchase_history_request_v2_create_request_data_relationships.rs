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
pub struct SandboxTestersClearPurchaseHistoryRequestV2CreateRequestDataRelationships {
    #[serde(rename = "sandboxTesters")]
    pub sandbox_testers: Box<models::SandboxTestersClearPurchaseHistoryRequestV2CreateRequestDataRelationshipsSandboxTesters>,
}

impl SandboxTestersClearPurchaseHistoryRequestV2CreateRequestDataRelationships {
    pub fn new(sandbox_testers: models::SandboxTestersClearPurchaseHistoryRequestV2CreateRequestDataRelationshipsSandboxTesters) -> SandboxTestersClearPurchaseHistoryRequestV2CreateRequestDataRelationships {
        SandboxTestersClearPurchaseHistoryRequestV2CreateRequestDataRelationships {
            sandbox_testers: Box::new(sandbox_testers),
        }
    }
}

