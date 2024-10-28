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
pub struct AppInfoAttributes {
    #[serde(rename = "appStoreState", skip_serializing_if = "Option::is_none")]
    pub app_store_state: Option<models::AppStoreVersionState>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
    #[serde(rename = "appStoreAgeRating", skip_serializing_if = "Option::is_none")]
    pub app_store_age_rating: Option<models::AppStoreAgeRating>,
    #[serde(rename = "australiaAgeRating", skip_serializing_if = "Option::is_none")]
    pub australia_age_rating: Option<AustraliaAgeRating>,
    #[serde(rename = "brazilAgeRating", skip_serializing_if = "Option::is_none")]
    pub brazil_age_rating: Option<models::BrazilAgeRating>,
    #[serde(rename = "brazilAgeRatingV2", skip_serializing_if = "Option::is_none")]
    pub brazil_age_rating_v2: Option<BrazilAgeRatingV2>,
    #[serde(rename = "koreaAgeRating", skip_serializing_if = "Option::is_none")]
    pub korea_age_rating: Option<KoreaAgeRating>,
    #[serde(rename = "kidsAgeBand", skip_serializing_if = "Option::is_none")]
    pub kids_age_band: Option<models::KidsAgeBand>,
}

impl AppInfoAttributes {
    pub fn new() -> AppInfoAttributes {
        AppInfoAttributes {
            app_store_state: None,
            state: None,
            app_store_age_rating: None,
            australia_age_rating: None,
            brazil_age_rating: None,
            brazil_age_rating_v2: None,
            korea_age_rating: None,
            kids_age_band: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "ACCEPTED")]
    Accepted,
    #[serde(rename = "DEVELOPER_REJECTED")]
    DeveloperRejected,
    #[serde(rename = "IN_REVIEW")]
    InReview,
    #[serde(rename = "PENDING_RELEASE")]
    PendingRelease,
    #[serde(rename = "PREPARE_FOR_SUBMISSION")]
    PrepareForSubmission,
    #[serde(rename = "READY_FOR_DISTRIBUTION")]
    ReadyForDistribution,
    #[serde(rename = "READY_FOR_REVIEW")]
    ReadyForReview,
    #[serde(rename = "REJECTED")]
    Rejected,
    #[serde(rename = "REPLACED_WITH_NEW_INFO")]
    ReplacedWithNewInfo,
    #[serde(rename = "WAITING_FOR_REVIEW")]
    WaitingForReview,
}

impl Default for State {
    fn default() -> State {
        Self::Accepted
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AustraliaAgeRating {
    #[serde(rename = "FIFTEEN")]
    Fifteen,
    #[serde(rename = "EIGHTEEN")]
    Eighteen,
}

impl Default for AustraliaAgeRating {
    fn default() -> AustraliaAgeRating {
        Self::Fifteen
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BrazilAgeRatingV2 {
    #[serde(rename = "SELF_RATED_L")]
    SelfRatedL,
    #[serde(rename = "SELF_RATED_TEN")]
    SelfRatedTen,
    #[serde(rename = "SELF_RATED_TWELVE")]
    SelfRatedTwelve,
    #[serde(rename = "SELF_RATED_FOURTEEN")]
    SelfRatedFourteen,
    #[serde(rename = "SELF_RATED_SIXTEEN")]
    SelfRatedSixteen,
    #[serde(rename = "SELF_RATED_EIGHTEEN")]
    SelfRatedEighteen,
    #[serde(rename = "OFFICIAL_L")]
    OfficialL,
    #[serde(rename = "OFFICIAL_TEN")]
    OfficialTen,
    #[serde(rename = "OFFICIAL_TWELVE")]
    OfficialTwelve,
    #[serde(rename = "OFFICIAL_FOURTEEN")]
    OfficialFourteen,
    #[serde(rename = "OFFICIAL_SIXTEEN")]
    OfficialSixteen,
    #[serde(rename = "OFFICIAL_EIGHTEEN")]
    OfficialEighteen,
}

impl Default for BrazilAgeRatingV2 {
    fn default() -> BrazilAgeRatingV2 {
        Self::SelfRatedL
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum KoreaAgeRating {
    #[serde(rename = "ALL")]
    All,
    #[serde(rename = "TWELVE")]
    Twelve,
    #[serde(rename = "FIFTEEN")]
    Fifteen,
    #[serde(rename = "NINETEEN")]
    Nineteen,
    #[serde(rename = "NOT_APPLICABLE")]
    NotApplicable,
}

impl Default for KoreaAgeRating {
    fn default() -> KoreaAgeRating {
        Self::All
    }
}

