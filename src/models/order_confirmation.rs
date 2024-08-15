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

/// OrderConfirmation : Represents the confirmation for an order that was just created.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderConfirmation {
    /// Representing trade action; currently supports buy and sell. buy OrderActionBuy sell OrderActionSell  OrderActionUnknown
    #[serde(rename = "action")]
    pub action: Action,
    #[serde(rename = "client_order_id")]
    pub client_order_id: String,
    /// Date and time in the ISO 8601 spec. Example: 2022-11-30T15:00:00Z
    #[serde(rename = "created_time", skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    /// Date and time in the ISO 8601 spec. Example: 2022-11-30T15:00:00Z
    #[serde(rename = "expiration_time", skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    /// The no price for this order in cents.
    #[serde(rename = "no_price")]
    pub no_price: i64,
    /// Unique identifier for orders.
    #[serde(rename = "order_id")]
    pub order_id: String,
    /// Representing direction of the order; currently supports yes and no. yes SIDE_YES no SIDE_NO  SIDE_UNSET
    #[serde(rename = "side")]
    pub side: Side,
    /// The current status of a given order. resting OrderStatusResting canceled OrderStatusCanceled executed OrderStatusExecuted pending OrderStatusPending  Will be used for order queue to represent orders that haven't been matched yet.
    #[serde(rename = "status")]
    pub status: Status,
    /// Unique identifier for markets.
    #[serde(rename = "ticker")]
    pub ticker: String,
    /// Representing order type; currently supports \"market\" and \"limit\".  OrderTypeUnknown market OrderTypeMarket limit OrderTypeLimit
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// The yes price for this order in cents.
    #[serde(rename = "yes_price")]
    pub yes_price: i64,
}

impl OrderConfirmation {
    /// Represents the confirmation for an order that was just created.
    pub fn new(action: Action, client_order_id: String, no_price: i64, order_id: String, side: Side, status: Status, ticker: String, r#type: Type, yes_price: i64) -> OrderConfirmation {
        OrderConfirmation {
            action,
            client_order_id,
            created_time: None,
            expiration_time: None,
            no_price,
            order_id,
            side,
            status,
            ticker,
            r#type,
            user_id: None,
            yes_price,
        }
    }
}
/// Representing trade action; currently supports buy and sell. buy OrderActionBuy sell OrderActionSell  OrderActionUnknown
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "buy")]
    Buy,
    #[serde(rename = "sell")]
    Sell,
    #[serde(rename = "")]
    Empty,
}

impl Default for Action {
    fn default() -> Action {
        Self::Buy
    }
}
/// Representing direction of the order; currently supports yes and no. yes SIDE_YES no SIDE_NO  SIDE_UNSET
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Side {
    #[serde(rename = "yes")]
    Yes,
    #[serde(rename = "no")]
    No,
    #[serde(rename = "")]
    Empty,
}

impl Default for Side {
    fn default() -> Side {
        Self::Yes
    }
}
/// The current status of a given order. resting OrderStatusResting canceled OrderStatusCanceled executed OrderStatusExecuted pending OrderStatusPending  Will be used for order queue to represent orders that haven't been matched yet.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "resting")]
    Resting,
    #[serde(rename = "canceled")]
    Canceled,
    #[serde(rename = "executed")]
    Executed,
    #[serde(rename = "pending")]
    Pending,
}

impl Default for Status {
    fn default() -> Status {
        Self::Resting
    }
}
/// Representing order type; currently supports \"market\" and \"limit\".  OrderTypeUnknown market OrderTypeMarket limit OrderTypeLimit
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "market")]
    Market,
    #[serde(rename = "limit")]
    Limit,
}

impl Default for Type {
    fn default() -> Type {
        Self::Empty
    }
}

