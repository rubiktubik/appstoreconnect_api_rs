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
pub struct AppInfoRelationships {
    #[serde(rename = "app", skip_serializing_if = "Option::is_none")]
    pub app: Option<Box<models::AlternativeDistributionKeyCreateRequestDataRelationshipsApp>>,
    #[serde(rename = "ageRatingDeclaration", skip_serializing_if = "Option::is_none")]
    pub age_rating_declaration: Option<Box<models::AppInfoRelationshipsAgeRatingDeclaration>>,
    #[serde(rename = "appInfoLocalizations", skip_serializing_if = "Option::is_none")]
    pub app_info_localizations: Option<Box<models::AppInfoRelationshipsAppInfoLocalizations>>,
    #[serde(rename = "primaryCategory", skip_serializing_if = "Option::is_none")]
    pub primary_category: Option<Box<models::AppCategoryRelationshipsParent>>,
    #[serde(rename = "primarySubcategoryOne", skip_serializing_if = "Option::is_none")]
    pub primary_subcategory_one: Option<Box<models::AppCategoryRelationshipsParent>>,
    #[serde(rename = "primarySubcategoryTwo", skip_serializing_if = "Option::is_none")]
    pub primary_subcategory_two: Option<Box<models::AppCategoryRelationshipsParent>>,
    #[serde(rename = "secondaryCategory", skip_serializing_if = "Option::is_none")]
    pub secondary_category: Option<Box<models::AppCategoryRelationshipsParent>>,
    #[serde(rename = "secondarySubcategoryOne", skip_serializing_if = "Option::is_none")]
    pub secondary_subcategory_one: Option<Box<models::AppCategoryRelationshipsParent>>,
    #[serde(rename = "secondarySubcategoryTwo", skip_serializing_if = "Option::is_none")]
    pub secondary_subcategory_two: Option<Box<models::AppCategoryRelationshipsParent>>,
}

impl AppInfoRelationships {
    pub fn new() -> AppInfoRelationships {
        AppInfoRelationships {
            app: None,
            age_rating_declaration: None,
            app_info_localizations: None,
            primary_category: None,
            primary_subcategory_one: None,
            primary_subcategory_two: None,
            secondary_category: None,
            secondary_subcategory_one: None,
            secondary_subcategory_two: None,
        }
    }
}

