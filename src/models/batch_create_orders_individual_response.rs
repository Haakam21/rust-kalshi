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
pub struct BatchCreateOrdersIndividualResponse {
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<Box<models::JsonError>>,
    #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
    pub order: Option<Box<models::OrderConfirmation>>,
}

impl BatchCreateOrdersIndividualResponse {
    pub fn new() -> BatchCreateOrdersIndividualResponse {
        BatchCreateOrdersIndividualResponse {
            error: None,
            order: None,
        }
    }
}

