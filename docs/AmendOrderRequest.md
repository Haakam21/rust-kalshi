# AmendOrderRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**action** | **String** | Specifies if this is a buy or sell order. Cannot be amended and is validated against original order. | 
**client_order_id** | **String** |  | 
**count** | **i32** | Number of contracts to be bought or sold. This is the max number of possible filled contracts. | 
**no_price** | Option<**i64**> | Submitting price of the No side of the trade, in cents. Exactly one of yes_price and no_price must be passed. If both prices are passed, return 400. | [optional]
**side** | **String** | Specifies if this is a 'yes' or 'no' order. Cannot be amended and is validated against original order. | 
**ticker** | **String** | The ticker of the market the order will be placed in. Cannot be amended and is validated against original order. | 
**updated_client_order_id** | **String** |  | 
**yes_price** | Option<**i64**> | Submitting price of the Yes side of the trade, in cents. Exactly one of yes_price and no_price must be passed. If both prices are passed, return 400. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


