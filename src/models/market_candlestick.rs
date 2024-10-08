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
pub struct MarketCandlestick {
    /// Unix timestamp for the inclusive end of the candlestick period.
    #[serde(rename = "end_period_ts")]
    pub end_period_ts: i64,
    /// Number of contracts bought on the market by end of the candlestick period (end_period_ts).
    #[serde(rename = "open_interest")]
    pub open_interest: i64,
    #[serde(rename = "price")]
    pub price: Box<models::PriceDistribution>,
    /// Number of contracts bought on the market during the candlestick period.
    #[serde(rename = "volume")]
    pub volume: i64,
    #[serde(rename = "yes_ask")]
    pub yes_ask: Box<models::BidAskDistribution>,
    #[serde(rename = "yes_bid")]
    pub yes_bid: Box<models::BidAskDistribution>,
}

impl MarketCandlestick {
    pub fn new(end_period_ts: i64, open_interest: i64, price: models::PriceDistribution, volume: i64, yes_ask: models::BidAskDistribution, yes_bid: models::BidAskDistribution) -> MarketCandlestick {
        MarketCandlestick {
            end_period_ts,
            open_interest,
            price: Box::new(price),
            volume,
            yes_ask: Box::new(yes_ask),
            yes_bid: Box::new(yes_bid),
        }
    }
}

