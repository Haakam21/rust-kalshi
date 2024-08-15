# PublicTrade

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**count** | **i32** | Number of contracts to be bought or sold. | 
**created_time** | **String** | Date and time in the ISO 8601 spec. Example: 2022-11-30T15:00:00Z | 
**no_price** | **i64** | No price for this trade in cents. | 
**taker_side** | **String** | Side for the taker of this trade. yes SIDE_YES no SIDE_NO  SIDE_UNSET | 
**ticker** | **String** | Unique identifier for markets. | 
**trade_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique identifier for this trade. | 
**yes_price** | **i64** | Yes price for this trade in cents. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


