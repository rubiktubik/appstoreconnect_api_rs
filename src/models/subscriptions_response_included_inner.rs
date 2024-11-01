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
pub enum SubscriptionsResponseIncludedInner {
    SubscriptionLocalization(Box<models::SubscriptionLocalization>),
    SubscriptionAppStoreReviewScreenshot(Box<models::SubscriptionAppStoreReviewScreenshot>),
    SubscriptionGroup(Box<models::SubscriptionGroup>),
    SubscriptionIntroductoryOffer(Box<models::SubscriptionIntroductoryOffer>),
    SubscriptionPromotionalOffer(Box<models::SubscriptionPromotionalOffer>),
    SubscriptionOfferCode(Box<models::SubscriptionOfferCode>),
    SubscriptionPrice(Box<models::SubscriptionPrice>),
    PromotedPurchase(Box<models::PromotedPurchase>),
    SubscriptionAvailability(Box<models::SubscriptionAvailability>),
    WinBackOffer(Box<models::WinBackOffer>),
    SubscriptionImage(Box<models::SubscriptionImage>),
}

impl Default for SubscriptionsResponseIncludedInner {
    fn default() -> Self {
        Self::SubscriptionLocalization(Default::default())
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "subscriptionLocalizations")]
    SubscriptionLocalizations,
    #[serde(rename = "subscriptionAppStoreReviewScreenshots")]
    SubscriptionAppStoreReviewScreenshots,
    #[serde(rename = "subscriptionGroups")]
    SubscriptionGroups,
    #[serde(rename = "subscriptionIntroductoryOffers")]
    SubscriptionIntroductoryOffers,
    #[serde(rename = "subscriptionPromotionalOffers")]
    SubscriptionPromotionalOffers,
    #[serde(rename = "subscriptionOfferCodes")]
    SubscriptionOfferCodes,
    #[serde(rename = "subscriptionPrices")]
    SubscriptionPrices,
    #[serde(rename = "promotedPurchases")]
    PromotedPurchases,
    #[serde(rename = "subscriptionAvailabilities")]
    SubscriptionAvailabilities,
    #[serde(rename = "winBackOffers")]
    WinBackOffers,
    #[serde(rename = "subscriptionImages")]
    SubscriptionImages,
}

impl Default for Type {
    fn default() -> Type {
        Self::SubscriptionLocalizations
    }
}

