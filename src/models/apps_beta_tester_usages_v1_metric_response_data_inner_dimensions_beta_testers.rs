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
pub struct AppsBetaTesterUsagesV1MetricResponseDataInnerDimensionsBetaTesters {
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<Box<models::AppsBetaTesterUsagesV1MetricResponseDataInnerDimensionsBetaTestersLinks>>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
}

impl AppsBetaTesterUsagesV1MetricResponseDataInnerDimensionsBetaTesters {
    pub fn new() -> AppsBetaTesterUsagesV1MetricResponseDataInnerDimensionsBetaTesters {
        AppsBetaTesterUsagesV1MetricResponseDataInnerDimensionsBetaTesters {
            links: None,
            data: None,
        }
    }
}

