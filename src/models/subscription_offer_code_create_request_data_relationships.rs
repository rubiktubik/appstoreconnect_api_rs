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
pub struct SubscriptionOfferCodeCreateRequestDataRelationships {
    #[serde(rename = "subscription")]
    pub subscription: Box<models::SubscriptionAppStoreReviewScreenshotCreateRequestDataRelationshipsSubscription>,
    #[serde(rename = "prices")]
    pub prices: Box<models::SubscriptionOfferCodeCreateRequestDataRelationshipsPrices>,
}

impl SubscriptionOfferCodeCreateRequestDataRelationships {
    pub fn new(subscription: models::SubscriptionAppStoreReviewScreenshotCreateRequestDataRelationshipsSubscription, prices: models::SubscriptionOfferCodeCreateRequestDataRelationshipsPrices) -> SubscriptionOfferCodeCreateRequestDataRelationships {
        SubscriptionOfferCodeCreateRequestDataRelationships {
            subscription: Box::new(subscription),
            prices: Box::new(prices),
        }
    }
}
