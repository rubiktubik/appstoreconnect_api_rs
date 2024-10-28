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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AppsResponseIncludedInner {
    AppEncryptionDeclaration(Box<models::AppEncryptionDeclaration>),
    CiProduct(Box<models::CiProduct>),
    BetaGroup(Box<models::BetaGroup>),
    AppStoreVersion(Box<models::AppStoreVersion>),
    PrereleaseVersion(Box<models::PrereleaseVersion>),
    BetaAppLocalization(Box<models::BetaAppLocalization>),
    Build(Box<models::Build>),
    BetaLicenseAgreement(Box<models::BetaLicenseAgreement>),
    BetaAppReviewDetail(Box<models::BetaAppReviewDetail>),
    AppInfo(Box<models::AppInfo>),
    AppClip(Box<models::AppClip>),
    EndUserLicenseAgreement(Box<models::EndUserLicenseAgreement>),
    AppPreOrder(Box<models::AppPreOrder>),
    InAppPurchase(Box<models::InAppPurchase>),
    SubscriptionGroup(Box<models::SubscriptionGroup>),
    GameCenterEnabledVersion(Box<models::GameCenterEnabledVersion>),
    AppCustomProductPage(Box<models::AppCustomProductPage>),
    InAppPurchaseV2(Box<models::InAppPurchaseV2>),
    PromotedPurchase(Box<models::PromotedPurchase>),
    AppEvent(Box<models::AppEvent>),
    ReviewSubmission(Box<models::ReviewSubmission>),
    SubscriptionGracePeriod(Box<models::SubscriptionGracePeriod>),
    GameCenterDetail(Box<models::GameCenterDetail>),
    AppStoreVersionExperimentV2(Box<models::AppStoreVersionExperimentV2>),
}

impl Default for AppsResponseIncludedInner {
    fn default() -> Self {
        Self::AppEncryptionDeclaration(Default::default())
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "appEncryptionDeclarations")]
    AppEncryptionDeclarations,
    #[serde(rename = "ciProducts")]
    CiProducts,
    #[serde(rename = "betaGroups")]
    BetaGroups,
    #[serde(rename = "appStoreVersions")]
    AppStoreVersions,
    #[serde(rename = "preReleaseVersions")]
    PreReleaseVersions,
    #[serde(rename = "betaAppLocalizations")]
    BetaAppLocalizations,
    #[serde(rename = "builds")]
    Builds,
    #[serde(rename = "betaLicenseAgreements")]
    BetaLicenseAgreements,
    #[serde(rename = "betaAppReviewDetails")]
    BetaAppReviewDetails,
    #[serde(rename = "appInfos")]
    AppInfos,
    #[serde(rename = "appClips")]
    AppClips,
    #[serde(rename = "endUserLicenseAgreements")]
    EndUserLicenseAgreements,
    #[serde(rename = "appPreOrders")]
    AppPreOrders,
    #[serde(rename = "inAppPurchases")]
    InAppPurchases,
    #[serde(rename = "subscriptionGroups")]
    SubscriptionGroups,
    #[serde(rename = "gameCenterEnabledVersions")]
    GameCenterEnabledVersions,
    #[serde(rename = "appCustomProductPages")]
    AppCustomProductPages,
    #[serde(rename = "promotedPurchases")]
    PromotedPurchases,
    #[serde(rename = "appEvents")]
    AppEvents,
    #[serde(rename = "reviewSubmissions")]
    ReviewSubmissions,
    #[serde(rename = "subscriptionGracePeriods")]
    SubscriptionGracePeriods,
    #[serde(rename = "gameCenterDetails")]
    GameCenterDetails,
    #[serde(rename = "appStoreVersionExperiments")]
    AppStoreVersionExperiments,
}

impl Default for Type {
    fn default() -> Type {
        Self::AppEncryptionDeclarations
    }
}
