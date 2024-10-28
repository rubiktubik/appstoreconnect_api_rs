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
pub struct UserInvitationRelationships {
    #[serde(rename = "visibleApps", skip_serializing_if = "Option::is_none")]
    pub visible_apps: Option<Box<models::BetaTesterRelationshipsApps>>,
}

impl UserInvitationRelationships {
    pub fn new() -> UserInvitationRelationships {
        UserInvitationRelationships {
            visible_apps: None,
        }
    }
}
