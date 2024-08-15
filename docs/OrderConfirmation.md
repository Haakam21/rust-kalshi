# OrderConfirmation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**action** | **String** | Representing trade action; currently supports buy and sell. buy OrderActionBuy sell OrderActionSell  OrderActionUnknown | 
**client_order_id** | **String** |  | 
**created_time** | Option<**String**> | Date and time in the ISO 8601 spec. Example: 2022-11-30T15:00:00Z | [optional]
**expiration_time** | Option<**String**> | Date and time in the ISO 8601 spec. Example: 2022-11-30T15:00:00Z | [optional]
**no_price** | **i64** | The no price for this order in cents. | 
**order_id** | **String** | Unique identifier for orders. | 
**side** | **String** | Representing direction of the order; currently supports yes and no. yes SIDE_YES no SIDE_NO  SIDE_UNSET | 
**status** | **String** | The current status of a given order. resting OrderStatusResting canceled OrderStatusCanceled executed OrderStatusExecuted pending OrderStatusPending  Will be used for order queue to represent orders that haven't been matched yet. | 
**ticker** | **String** | Unique identifier for markets. | 
**r#type** | **String** | Representing order type; currently supports \"market\" and \"limit\".  OrderTypeUnknown market OrderTypeMarket limit OrderTypeLimit | 
**user_id** | Option<**String**> |  | [optional]
**yes_price** | **i64** | The yes price for this order in cents. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


