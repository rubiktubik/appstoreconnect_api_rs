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
pub struct AppClipHeaderImageCreateRequestDataRelationships {
    #[serde(rename = "appClipDefaultExperienceLocalization")]
    pub app_clip_default_experience_localization: Box<models::AppClipHeaderImageCreateRequestDataRelationshipsAppClipDefaultExperienceLocalization>,
}

impl AppClipHeaderImageCreateRequestDataRelationships {
    pub fn new(app_clip_default_experience_localization: models::AppClipHeaderImageCreateRequestDataRelationshipsAppClipDefaultExperienceLocalization) -> AppClipHeaderImageCreateRequestDataRelationships {
        AppClipHeaderImageCreateRequestDataRelationships {
            app_clip_default_experience_localization: Box::new(app_clip_default_experience_localization),
        }
    }
}

