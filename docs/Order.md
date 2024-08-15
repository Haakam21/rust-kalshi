# Order

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**action** | **String** | Representing trade action; currently supports buy and sell. buy OrderActionBuy sell OrderActionSell  OrderActionUnknown | 
**amend_count** | Option<**i32**> | The amendment delta throughout the lifecycle of the order (contract units). | [optional]
**amend_taker_fill_count** | Option<**i32**> | The size of filled taker orders (contract units) as a result of an amendment | [optional]
**client_order_id** | **String** |  | 
**close_cancel_count** | Option<**i32**> | The size of resting orders canceled because of market close (contract units). | [optional]
**created_time** | Option<**String**> | Date and time in the ISO 8601 spec. Example: 2022-11-30T15:00:00Z | [optional]
**decrease_count** | Option<**i32**> | The reduction in the size of resting for orders (contract units). | [optional]
**expiration_time** | Option<**String**> | Date and time in the ISO 8601 spec. Example: 2022-11-30T15:00:00Z | [optional]
**fcc_cancel_count** | Option<**i32**> | The size of resting contracts canceled because of exchange operations (contract units). | [optional]
**last_update_time** | Option<**String**> | Date and time in the ISO 8601 spec. Example: 2022-11-30T15:00:00Z | [optional]
**maker_fees** | Option<**i64**> | Fees paid on filled maker contracts, in cents. | [optional]
**maker_fill_cost** | Option<**i64**> | The cost of filled maker orders in cents. | [optional]
**maker_fill_count** | Option<**i32**> | The size of filled maker orders (contract units). | [optional]
**no_price** | **i64** | Submitting price of the No side of the trade, in cents. Exactly one of yes_price and no_price must be passed. If both prices are passed, return 400. | 
**order_id** | **String** | Unique identifier for orders. | 
**place_count** | Option<**i32**> | the size of placed maker orders (contract units). | [optional]
**queue_position** | Option<**i32**> | Position in the priority queue at a given price level | [optional]
**remaining_count** | Option<**i32**> | The size of the remaining resting orders (contract units). | [optional]
**side** | **String** | Representing direction of the order; currently supports yes and no. yes SIDE_YES no SIDE_NO  SIDE_UNSET | 
**status** | **String** | The current status of this order. resting OrderStatusResting canceled OrderStatusCanceled executed OrderStatusExecuted pending OrderStatusPending  Will be used for order queue to represent orders that haven't been matched yet. | 
**taker_fees** | Option<**i64**> | Fees paid on filled taker contracts, in cents. | [optional]
**taker_fill_cost** | Option<**i64**> | The cost of filled taker orders in cents. | [optional]
**taker_fill_count** | Option<**i32**> | The size of filled taker orders (contract units) | [optional]
**taker_self_trade_cancel_count** | Option<**i32**> | The reduction in the size of a taker order due to self-trade prevention cancellation (contract units). Will be zero for orders placed before the introduction of this field. | [optional]
**ticker** | **String** | Unique identifier for markets. | 
**r#type** | **String** | Representing order type; currently supports \"market\" and \"limit\".  OrderTypeUnknown market OrderTypeMarket limit OrderTypeLimit | 
**user_id** | Option<**String**> |  | [optional]
**yes_price** | **i64** | The yes price for this order in cents. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


