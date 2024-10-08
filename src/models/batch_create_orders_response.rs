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
pub struct BatchCreateOrdersResponse {
    /// An array of responses corresponding to orders in the request.
    #[serde(rename = "orders")]
    pub orders: Vec<models::BatchCreateOrdersIndividualResponse>,
}

impl BatchCreateOrdersResponse {
    pub fn new(orders: Vec<models::BatchCreateOrdersIndividualResponse>) -> BatchCreateOrdersResponse {
        BatchCreateOrdersResponse {
            orders,
        }
    }
}

