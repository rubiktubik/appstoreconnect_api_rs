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
pub struct InAppPurchaseImageAttributes {
    #[serde(rename = "fileSize", skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i32>,
    #[serde(rename = "fileName", skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(rename = "sourceFileChecksum", skip_serializing_if = "Option::is_none")]
    pub source_file_checksum: Option<String>,
    #[serde(rename = "assetToken", skip_serializing_if = "Option::is_none")]
    pub asset_token: Option<String>,
    #[serde(rename = "imageAsset", skip_serializing_if = "Option::is_none")]
    pub image_asset: Option<Box<models::ImageAsset>>,
    #[serde(rename = "uploadOperations", skip_serializing_if = "Option::is_none")]
    pub upload_operations: Option<Vec<models::UploadOperation>>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
}

impl InAppPurchaseImageAttributes {
    pub fn new() -> InAppPurchaseImageAttributes {
        InAppPurchaseImageAttributes {
            file_size: None,
            file_name: None,
            source_file_checksum: None,
            asset_token: None,
            image_asset: None,
            upload_operations: None,
            state: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "AWAITING_UPLOAD")]
    AwaitingUpload,
    #[serde(rename = "UPLOAD_COMPLETE")]
    UploadComplete,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "PREPARE_FOR_SUBMISSION")]
    PrepareForSubmission,
    #[serde(rename = "WAITING_FOR_REVIEW")]
    WaitingForReview,
    #[serde(rename = "APPROVED")]
    Approved,
    #[serde(rename = "REJECTED")]
    Rejected,
}

impl Default for State {
    fn default() -> State {
        Self::AwaitingUpload
    }
}
