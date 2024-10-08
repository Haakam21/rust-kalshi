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
pub struct AmendOrderResponse {
    #[serde(rename = "old_order", skip_serializing_if = "Option::is_none")]
    pub old_order: Option<Box<models::Order>>,
    #[serde(rename = "order")]
    pub order: Box<models::Order>,
}

impl AmendOrderResponse {
    pub fn new(order: models::Order) -> AmendOrderResponse {
        AmendOrderResponse {
            old_order: None,
            order: Box::new(order),
        }
    }
}

