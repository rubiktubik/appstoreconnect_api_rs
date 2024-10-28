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
pub struct SubscriptionIntroductoryOfferCreateRequestDataRelationships {
    #[serde(rename = "subscription")]
    pub subscription: Box<models::SubscriptionAppStoreReviewScreenshotCreateRequestDataRelationshipsSubscription>,
    #[serde(rename = "territory", skip_serializing_if = "Option::is_none")]
    pub territory: Option<Box<models::AppPricePointV3RelationshipsTerritory>>,
    #[serde(rename = "subscriptionPricePoint", skip_serializing_if = "Option::is_none")]
    pub subscription_price_point: Option<Box<models::SubscriptionIntroductoryOfferRelationshipsSubscriptionPricePoint>>,
}

impl SubscriptionIntroductoryOfferCreateRequestDataRelationships {
    pub fn new(subscription: models::SubscriptionAppStoreReviewScreenshotCreateRequestDataRelationshipsSubscription) -> SubscriptionIntroductoryOfferCreateRequestDataRelationships {
        SubscriptionIntroductoryOfferCreateRequestDataRelationships {
            subscription: Box::new(subscription),
            territory: None,
            subscription_price_point: None,
        }
    }
}

