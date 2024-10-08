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
pub struct GetSeriesResponse {
    #[serde(rename = "series", skip_serializing_if = "Option::is_none")]
    pub series: Option<Box<models::Series>>,
}

impl GetSeriesResponse {
    pub fn new() -> GetSeriesResponse {
        GetSeriesResponse {
            series: None,
        }
    }
}

