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
pub struct EventData {
    /// Deprecated: Event category. Use the series level property instead.
    #[serde(rename = "category")]
    pub category: String,
    /// Unique identifier for events.
    #[serde(rename = "event_ticker")]
    pub event_ticker: String,
    /// The markets that are linked to this event. Will be filled only if the query parameter \"with_nested_markets\" is equal \"true\".
    #[serde(rename = "markets", skip_serializing_if = "Option::is_none")]
    pub markets: Option<Vec<models::Market>>,
    /// If true then the event is mutually exclusive.
    #[serde(rename = "mutually_exclusive")]
    pub mutually_exclusive: bool,
    /// Unique identifier for series.
    #[serde(rename = "series_ticker")]
    pub series_ticker: String,
    /// Date and time in the ISO 8601 spec. Example: 2022-11-30T15:00:00Z
    #[serde(rename = "strike_date", skip_serializing_if = "Option::is_none")]
    pub strike_date: Option<String>,
    /// The strike period for this event. This will be filled when the event strike is not a date. If it is a date then the 'strike_date' field should be filled instead.
    #[serde(rename = "strike_period", skip_serializing_if = "Option::is_none")]
    pub strike_period: Option<String>,
    /// Shortened title.
    #[serde(rename = "sub_title")]
    pub sub_title: String,
    /// Event title.
    #[serde(rename = "title")]
    pub title: String,
}

impl EventData {
    pub fn new(category: String, event_ticker: String, mutually_exclusive: bool, series_ticker: String, sub_title: String, title: String) -> EventData {
        EventData {
            category,
            event_ticker,
            markets: None,
            mutually_exclusive,
            series_ticker,
            strike_date: None,
            strike_period: None,
            sub_title,
            title,
        }
    }
}

