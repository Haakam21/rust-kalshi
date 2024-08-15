# Market

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**can_close_early** | **bool** | If true then this market can close earlier then the time provided on close_time. | 
**cap_strike** | Option<**f64**> |  | [optional]
**category** | **String** | Category for this market. | 
**close_time** | **String** | Date and time in the ISO 8601 spec. Example: 2022-11-30T15:00:00Z | 
**custom_strike** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | Expiration value for each target that leads to a YES settlement.  Filled only if \"strike_type\" is \"custom\". | [optional]
**event_ticker** | **String** | Unique identifier for events. | 
**expected_expiration_time** | Option<**String**> | Date and time in the ISO 8601 spec. Example: 2022-11-30T15:00:00Z | [optional]
**expiration_time** | **String** | Date and time in the ISO 8601 spec. Example: 2022-11-30T15:00:00Z | 
**expiration_value** | **String** | The value that was considered for the settlement. | 
**fee_waiver_expiration_time** | Option<**String**> | Date and time in the ISO 8601 spec. Example: 2022-11-30T15:00:00Z | [optional]
**floor_strike** | Option<**f64**> |  | [optional]
**functional_strike** | Option<**String**> | Mapping from expiration values to settlement values of the YES/LONG side, in centi-cents.  Filled only if \"market_type\" is \"scalar\" and \"strike_type\" is \"functional\".  Ex. f(x) = max(0, min(10000, 500 * x))  A scalar market with this functional strike and an expiration value of 10 would have a settlement value on the YES/LONG side of 5000 centi cents. | [optional]
**last_price** | **i64** | Price for the last traded yes contract on this market. | 
**latest_expiration_time** | **String** | Date and time in the ISO 8601 spec. Example: 2022-11-30T15:00:00Z | 
**liquidity** | **i64** | Value for current offers in this market in cents. | 
**market_type** | Option<[**serde_json::Value**](.md)> | Identifies the type of market, which affects its payout and structure.  binary: Every binary market has two sides, YES and NO. If the market's \"payout criterion\" is satisfied, it pays out the notional value to holders of YES. Otherwise, it pays out the notional value to holders of NO.  scalar: Every scalar market has two sides, LONG and SHORT (although these might be referred to as YES/NO in some API endpoints). At settlement, each contract's notional value is split between LONG and SHORT as described in the market rules. | 
**no_ask** | **i64** | Price for the lowest NO sell offer on this market. | 
**no_bid** | **i64** | Price for the highest NO buy offer on this market. | 
**no_sub_title** | **String** | Shortened title for the no side of this market. | 
**notional_value** | **i64** | The total value of a single contract at settlement. | 
**open_interest** | **i64** | Number of contracts bought on this market disconsidering netting. | 
**open_time** | **String** | Date and time in the ISO 8601 spec. Example: 2022-11-30T15:00:00Z | 
**previous_price** | **i64** | Price for the last traded yes contract on this market a day ago. | 
**previous_yes_ask** | **i64** | Price for the lowest YES sell offer on this market a day ago. | 
**previous_yes_bid** | **i64** | Price for the highest YES buy offer on this market a day ago. | 
**response_price_units** | **String** | The units used to express all price related fields in this response, including: prices, bids/asks, liquidity, notional and settlement values. usd_cent MONEY_UNIT_USD_CENT usd_centi_cent MONEY_UNIT_USD_CENTI_CENT | 
**result** | **String** | Settlement result for this market. Filled only after determination. Omitted for scalar markets.  MARKET_RESULT_NO_RESULT yes MARKET_RESULT_YES no MARKET_RESULT_NO void MARKET_RESULT_VOID all_no RANGED_MARKET_RESULT_ALL_NO all_yes RANGED_MARKET_RESULT_ALL_YES | 
**risk_limit_cents** | **i64** | Risk limit for this market in cents. | 
**rules_primary** | **String** | A plain language description of the most important market terms. | 
**rules_secondary** | **String** | A plain language description of secondary market terms. | 
**settlement_timer_seconds** | **i32** | The amount of time after determination that the market settles (pays out). | 
**settlement_value** | Option<**i64**> | The settlement value of the YES/LONG side of the contract. Only filled after determination. | [optional]
**status** | Option<[**serde_json::Value**](.md)> | Represents the current status of a market. | 
**strike_type** | Option<**String**> | Strike type defines how the market strike (expiration value) is defined and evaluated.  greater: It will be a single number. For YES outcome the expiration value should be greater than \"floor_strike\".  greater_or_equal: It will be a single number. For YES outcome the expiration value should be greater OR EQUAL than \"floor_strike\".  less: It will be a single number. For YES outcome the expiration value should be less than \"cap_strike\".  less_or_equal: It will be a single number. For YES outcome the expiration value should be less OR EQUAL than \"cap_strike\".  between: It will be two numbers. For YES outcome the expiration value should be between inclusive \"floor_strike\" and \"cap_strike\", that means expiration value needs to be greater or equal \"floor_strike\" and less or equal \"cap_strike\".  functional: For scalar markets only. A mapping from expiration values to settlement values of the YES/LONG side will be in \"functional_strike\".  custom: It will be one or more non-numerical values. For YES outcome the expiration values should be equal to the values in \"custom_strike\". unknown MarketStrikeTypeUnknown greater MarketStrikeTypeGreater less MarketStrikeTypeLess greater_or_equal MarketStrikeTypeGreaterOrEqual less_or_equal MarketStrikeTypeLessOrEqual between MarketStrikeTypeBetween functional MarketStrikeTypeFunctional custom MarketStrikeTypeCustom | [optional]
**subtitle** | **String** | Deprecated: Shortened title for this market. Use \"yes_sub_title\" or \"no_sub_title\" instead. | 
**tick_size** | **i64** | The minimum price movement in the market. All limit order prices must be in denominations of the tick size. | 
**ticker** | **String** | Unique identifier for markets. | 
**title** | **String** | Full title describing this market. | 
**volume** | **i64** | Number of contracts bought on this market. | 
**volume_24h** | **i64** | Number of contracts bought on this market in the past day. | 
**yes_ask** | **i64** | Price for the lowest YES sell offer on this market. | 
**yes_bid** | **i64** | Price for the highest YES buy offer on this market. | 
**yes_sub_title** | **String** | Shortened title for the yes side of this market. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


