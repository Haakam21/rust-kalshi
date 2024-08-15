# MarketCandlestick

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**end_period_ts** | **i64** | Unix timestamp for the inclusive end of the candlestick period. | 
**open_interest** | **i64** | Number of contracts bought on the market by end of the candlestick period (end_period_ts). | 
**price** | [**models::PriceDistribution**](PriceDistribution.md) |  | 
**volume** | **i64** | Number of contracts bought on the market during the candlestick period. | 
**yes_ask** | [**models::BidAskDistribution**](BidAskDistribution.md) |  | 
**yes_bid** | [**models::BidAskDistribution**](BidAskDistribution.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


