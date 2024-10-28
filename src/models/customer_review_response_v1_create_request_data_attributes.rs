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
pub struct CustomerReviewResponseV1CreateRequestDataAttributes {
    #[serde(rename = "responseBody")]
    pub response_body: String,
}

impl CustomerReviewResponseV1CreateRequestDataAttributes {
    pub fn new(response_body: String) -> CustomerReviewResponseV1CreateRequestDataAttributes {
        CustomerReviewResponseV1CreateRequestDataAttributes {
            response_body,
        }
    }
}
