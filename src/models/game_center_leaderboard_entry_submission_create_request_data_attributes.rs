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
pub struct GameCenterLeaderboardEntrySubmissionCreateRequestDataAttributes {
    #[serde(rename = "bundleId")]
    pub bundle_id: String,
    #[serde(rename = "challengeIds", skip_serializing_if = "Option::is_none")]
    pub challenge_ids: Option<Vec<String>>,
    #[serde(rename = "context", skip_serializing_if = "Option::is_none")]
    pub context: Option<models::Decimal>,
    #[serde(rename = "scopedPlayerId")]
    pub scoped_player_id: String,
    #[serde(rename = "score")]
    pub score: models::Decimal,
    #[serde(rename = "submittedDate", skip_serializing_if = "Option::is_none")]
    pub submitted_date: Option<String>,
    #[serde(rename = "vendorIdentifier")]
    pub vendor_identifier: String,
}

impl GameCenterLeaderboardEntrySubmissionCreateRequestDataAttributes {
    pub fn new(bundle_id: String, scoped_player_id: String, score: models::Decimal, vendor_identifier: String) -> GameCenterLeaderboardEntrySubmissionCreateRequestDataAttributes {
        GameCenterLeaderboardEntrySubmissionCreateRequestDataAttributes {
            bundle_id,
            challenge_ids: None,
            context: None,
            scoped_player_id,
            score,
            submitted_date: None,
            vendor_identifier,
        }
    }
}

