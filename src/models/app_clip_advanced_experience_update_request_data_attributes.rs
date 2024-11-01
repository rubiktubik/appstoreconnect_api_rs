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
pub struct AppClipAdvancedExperienceUpdateRequestDataAttributes {
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<models::AppClipAction>,
    #[serde(rename = "isPoweredBy", skip_serializing_if = "Option::is_none")]
    pub is_powered_by: Option<bool>,
    #[serde(rename = "place", skip_serializing_if = "Option::is_none")]
    pub place: Option<Box<models::AppClipAdvancedExperienceAttributesPlace>>,
    #[serde(rename = "businessCategory", skip_serializing_if = "Option::is_none")]
    pub business_category: Option<BusinessCategory>,
    #[serde(rename = "defaultLanguage", skip_serializing_if = "Option::is_none")]
    pub default_language: Option<models::AppClipAdvancedExperienceLanguage>,
    #[serde(rename = "removed", skip_serializing_if = "Option::is_none")]
    pub removed: Option<bool>,
}

impl AppClipAdvancedExperienceUpdateRequestDataAttributes {
    pub fn new() -> AppClipAdvancedExperienceUpdateRequestDataAttributes {
        AppClipAdvancedExperienceUpdateRequestDataAttributes {
            action: None,
            is_powered_by: None,
            place: None,
            business_category: None,
            default_language: None,
            removed: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BusinessCategory {
    #[serde(rename = "AUTOMOTIVE")]
    Automotive,
    #[serde(rename = "BEAUTY")]
    Beauty,
    #[serde(rename = "BIKES")]
    Bikes,
    #[serde(rename = "BOOKS")]
    Books,
    #[serde(rename = "CASINO")]
    Casino,
    #[serde(rename = "EDUCATION")]
    Education,
    #[serde(rename = "EDUCATION_JAPAN")]
    EducationJapan,
    #[serde(rename = "ENTERTAINMENT")]
    Entertainment,
    #[serde(rename = "EV_CHARGER")]
    EvCharger,
    #[serde(rename = "FINANCIAL_USD")]
    FinancialUsd,
    #[serde(rename = "FINANCIAL_CNY")]
    FinancialCny,
    #[serde(rename = "FINANCIAL_GBP")]
    FinancialGbp,
    #[serde(rename = "FINANCIAL_JPY")]
    FinancialJpy,
    #[serde(rename = "FINANCIAL_EUR")]
    FinancialEur,
    #[serde(rename = "FITNESS")]
    Fitness,
    #[serde(rename = "FOOD_AND_DRINK")]
    FoodAndDrink,
    #[serde(rename = "GAS")]
    Gas,
    #[serde(rename = "GROCERY")]
    Grocery,
    #[serde(rename = "HEALTH_AND_MEDICAL")]
    HealthAndMedical,
    #[serde(rename = "HOTEL_AND_TRAVEL")]
    HotelAndTravel,
    #[serde(rename = "MUSIC")]
    Music,
    #[serde(rename = "PARKING")]
    Parking,
    #[serde(rename = "PET_SERVICES")]
    PetServices,
    #[serde(rename = "PROFESSIONAL_SERVICES")]
    ProfessionalServices,
    #[serde(rename = "SHOPPING")]
    Shopping,
    #[serde(rename = "TICKETING")]
    Ticketing,
    #[serde(rename = "TRANSIT")]
    Transit,
}

impl Default for BusinessCategory {
    fn default() -> BusinessCategory {
        Self::Automotive
    }
}

