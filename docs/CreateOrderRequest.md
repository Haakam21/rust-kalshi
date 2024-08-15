# CreateOrderRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**action** | **String** | Specifies if this is a buy or sell order. | 
**buy_max_cost** | Option<**i64**> | If type = market and action = buy, buy_max_cost represents the maximum cents that can be spent to acquire a position. | [optional]
**client_order_id** | **String** |  | 
**count** | **i32** | Number of contracts to be bought or sold. | 
**expiration_ts** | Option<**i64**> | Expiration time of the order, in unix seconds.  If this is not supplied, the order won't expire until explicitly cancelled. This is also known as Good 'Till Cancelled (GTC).  If the time is in the past, the order will attempt to partially or completely fill and the remaining unfilled quantity will be cancelled. This is also known as Immediate-or-Cancel (IOC).  If the time is in the future, the remaining unfilled quantity order will expire at the specified time. | [optional]
**no_price** | Option<**i64**> | Submitting price of the No side of the trade, in cents. Exactly one of yes_price and no_price must be passed. If both prices are passed, return 400. | [optional]
**sell_position_floor** | Option<**i32**> | SellPositionFloor will not let you flip position for a market order if set to 0. | [optional]
**side** | **String** | Specifies if this is a 'yes' or 'no' order. | 
**ticker** | **String** | The ticker of the market the order will be placed in. | 
**r#type** | **String** | Specifies if this is a \"market\" or a \"limit\" order. Note that either the Yes Price or the No Price must be provided for limit orders. | 
**yes_price** | Option<**i64**> | Submitting price of the Yes side of the trade, in cents. Exactly one of yes_price and no_price must be passed. If both prices are passed, return 400. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


