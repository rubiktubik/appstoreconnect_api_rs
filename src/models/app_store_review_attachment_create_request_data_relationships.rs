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
pub struct AppStoreReviewAttachmentCreateRequestDataRelationships {
    #[serde(rename = "appStoreReviewDetail")]
    pub app_store_review_detail: Box<models::AppStoreReviewAttachmentCreateRequestDataRelationshipsAppStoreReviewDetail>,
}

impl AppStoreReviewAttachmentCreateRequestDataRelationships {
    pub fn new(app_store_review_detail: models::AppStoreReviewAttachmentCreateRequestDataRelationshipsAppStoreReviewDetail) -> AppStoreReviewAttachmentCreateRequestDataRelationships {
        AppStoreReviewAttachmentCreateRequestDataRelationships {
            app_store_review_detail: Box::new(app_store_review_detail),
        }
    }
}
