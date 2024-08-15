# Fill

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**action** | **String** | Specifies if this is a buy or sell order. buy OrderActionBuy sell OrderActionSell  OrderActionUnknown | 
**count** | **i32** | Number of contracts to be bought or sold. | 
**created_time** | **String** | Date and time in the ISO 8601 spec. Example: 2022-11-30T15:00:00Z | 
**is_taker** | **bool** | If true then this fill was a taker. | 
**no_price** | **i64** | Fill price for the no side in cents. | 
**order_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique identifier for orders. | 
**side** | **String** | Specifies if this is a 'yes' or 'no' fill. yes SIDE_YES no SIDE_NO  SIDE_UNSET | 
**ticker** | **String** | Unique identifier for markets. | 
**trade_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique identifier for fills. | 
**yes_price** | **i64** | Fill price for the yes side in cents. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


