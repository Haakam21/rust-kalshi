/*
 * Kalshi Trade API
 *
 * This documentation describes Kalshi's trading API (known as Trade API v2). By using this API, you agree to Kalshi's Developer Agreement (https://kalshi.com/developer-agreement).
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: support@kalshi.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BidAskDistribution {
    /// Offer price on the market at the end of the candlestick period.
    #[serde(rename = "close")]
    pub close: i64,
    /// Highest offer price on the market during the candlestick period.
    #[serde(rename = "high")]
    pub high: i64,
    /// Lowest offer price on the market during the candlestick period.
    #[serde(rename = "low")]
    pub low: i64,
    /// Offer price on the market at the start of the candlestick period.
    #[serde(rename = "open")]
    pub open: i64,
}

impl BidAskDistribution {
    pub fn new(close: i64, high: i64, low: i64, open: i64) -> BidAskDistribution {
        BidAskDistribution {
            close,
            high,
            low,
            open,
        }
    }
}

