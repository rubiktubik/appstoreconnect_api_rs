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

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum GameCenterLeaderboardFormatter {
    #[serde(rename = "INTEGER")]
    Integer,
    #[serde(rename = "DECIMAL_POINT_1_PLACE")]
    DecimalPoint1Place,
    #[serde(rename = "DECIMAL_POINT_2_PLACE")]
    DecimalPoint2Place,
    #[serde(rename = "DECIMAL_POINT_3_PLACE")]
    DecimalPoint3Place,
    #[serde(rename = "ELAPSED_TIME_MILLISECOND")]
    ElapsedTimeMillisecond,
    #[serde(rename = "ELAPSED_TIME_CENTISECOND")]
    ElapsedTimeCentisecond,
    #[serde(rename = "ELAPSED_TIME_MINUTE")]
    ElapsedTimeMinute,
    #[serde(rename = "ELAPSED_TIME_SECOND")]
    ElapsedTimeSecond,
    #[serde(rename = "MONEY_POUND_DECIMAL")]
    MoneyPoundDecimal,
    #[serde(rename = "MONEY_POUND")]
    MoneyPound,
    #[serde(rename = "MONEY_DOLLAR_DECIMAL")]
    MoneyDollarDecimal,
    #[serde(rename = "MONEY_DOLLAR")]
    MoneyDollar,
    #[serde(rename = "MONEY_EURO_DECIMAL")]
    MoneyEuroDecimal,
    #[serde(rename = "MONEY_EURO")]
    MoneyEuro,
    #[serde(rename = "MONEY_FRANC_DECIMAL")]
    MoneyFrancDecimal,
    #[serde(rename = "MONEY_FRANC")]
    MoneyFranc,
    #[serde(rename = "MONEY_KRONER_DECIMAL")]
    MoneyKronerDecimal,
    #[serde(rename = "MONEY_KRONER")]
    MoneyKroner,
    #[serde(rename = "MONEY_YEN")]
    MoneyYen,

}

impl std::fmt::Display for GameCenterLeaderboardFormatter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Integer => write!(f, "INTEGER"),
            Self::DecimalPoint1Place => write!(f, "DECIMAL_POINT_1_PLACE"),
            Self::DecimalPoint2Place => write!(f, "DECIMAL_POINT_2_PLACE"),
            Self::DecimalPoint3Place => write!(f, "DECIMAL_POINT_3_PLACE"),
            Self::ElapsedTimeMillisecond => write!(f, "ELAPSED_TIME_MILLISECOND"),
            Self::ElapsedTimeCentisecond => write!(f, "ELAPSED_TIME_CENTISECOND"),
            Self::ElapsedTimeMinute => write!(f, "ELAPSED_TIME_MINUTE"),
            Self::ElapsedTimeSecond => write!(f, "ELAPSED_TIME_SECOND"),
            Self::MoneyPoundDecimal => write!(f, "MONEY_POUND_DECIMAL"),
            Self::MoneyPound => write!(f, "MONEY_POUND"),
            Self::MoneyDollarDecimal => write!(f, "MONEY_DOLLAR_DECIMAL"),
            Self::MoneyDollar => write!(f, "MONEY_DOLLAR"),
            Self::MoneyEuroDecimal => write!(f, "MONEY_EURO_DECIMAL"),
            Self::MoneyEuro => write!(f, "MONEY_EURO"),
            Self::MoneyFrancDecimal => write!(f, "MONEY_FRANC_DECIMAL"),
            Self::MoneyFranc => write!(f, "MONEY_FRANC"),
            Self::MoneyKronerDecimal => write!(f, "MONEY_KRONER_DECIMAL"),
            Self::MoneyKroner => write!(f, "MONEY_KRONER"),
            Self::MoneyYen => write!(f, "MONEY_YEN"),
        }
    }
}

impl Default for GameCenterLeaderboardFormatter {
    fn default() -> GameCenterLeaderboardFormatter {
        Self::Integer
    }
}

