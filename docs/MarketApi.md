# \MarketApi

All URIs are relative to *https://trading-api.kalshi.com/trade-api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_event**](MarketApi.md#get_event) | **GET** /events/{event_ticker} | GetEvent
[**get_events**](MarketApi.md#get_events) | **GET** /events | GetEvents
[**get_market**](MarketApi.md#get_market) | **GET** /markets/{ticker} | GetMarket
[**get_market_candlesticks**](MarketApi.md#get_market_candlesticks) | **GET** /series/{series_ticker}/markets/{ticker}/candlesticks | GetMarketCandlesticks
[**get_market_orderbook**](MarketApi.md#get_market_orderbook) | **GET** /markets/{ticker}/orderbook | GetMarketOrderbook
[**get_markets**](MarketApi.md#get_markets) | **GET** /markets | GetMarkets
[**get_series**](MarketApi.md#get_series) | **GET** /series/{series_ticker} | GetSeries
[**get_trades**](MarketApi.md#get_trades) | **GET** /markets/trades | GetTrades



## get_event

> models::GetEventResponse get_event(event_ticker, with_nested_markets)
GetEvent

Endpoint for getting data about an event by its ticker.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_ticker** | **String** | Should be filled with the ticker of the event. | [required] |
**with_nested_markets** | Option<**bool**> | If the markets belonging to the events should be added in the response as a nested field in this event. |  |

### Return type

[**models::GetEventResponse**](GetEventResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_events

> models::GetEventsResponse get_events(limit, cursor, status, series_ticker, with_nested_markets)
GetEvents

Endpoint for getting data about all events.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i64**> | Parameter to specify the number of results per page. Defaults to 100. |  |
**cursor** | Option<**String**> | The Cursor represents a pointer to the next page of records in the pagination. So this optional parameter, when filled, should be filled with the cursor string returned in a previous request to this end-point. Filling this would basically tell the api to get the next page containing the number of records passed on the limit parameter. On the other side not filling it tells the api you want to get the first page for another query. The cursor does not store any filters, so if any filter parameters like series_ticker was passed in the original query they must be passed again. |  |
**status** | Option<**String**> | Restricts the events to those with certain statuses, as a comma separated list. The following values are accepted: unopened, open, closed, settled. |  |
**series_ticker** | Option<**String**> | Series ticker to retrieve contracts for. |  |
**with_nested_markets** | Option<**bool**> | If the markets belonging to the events should be added in the response as a nested field in this event. |  |

### Return type

[**models::GetEventsResponse**](GetEventsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_market

> models::GetMarketResponse get_market(ticker)
GetMarket

Endpoint for getting data about a specific market.  The value for the ticker path parameter should match the ticker of the target market.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticker** | **String** | Market ticker for the market being retrieved. | [required] |

### Return type

[**models::GetMarketResponse**](GetMarketResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_market_candlesticks

> models::GetMarketCandlesticksResponse get_market_candlesticks(ticker, series_ticker, start_ts, end_ts, period_interval)
GetMarketCandlesticks

Endpoint for getting the historical candlesticks for a market.  The values for the series_ticker and ticker path parameters should match the series_ticker and ticker of the target market. The start_ts parameter will restrict candlesticks to those ending on or after provided timestamp. The end_ts parameter will restrict candlesticks to those ending on or before provided timestamp. The period_interval parameter determines the time period length of each candlestick.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticker** | **String** | Unique identifier for the market. | [required] |
**series_ticker** | **String** | Unique identifier for the series. | [required] |
**start_ts** | **i64** | Restricts the candlesticks to those covering time periods that end on or after this timestamp. | [required] |
**end_ts** | **i64** | Restricts the candlesticks to those covering time periods that end on or before this timestamp. Must be within 5000 period_intervals after start_ts. | [required] |
**period_interval** | **i32** | Specifies the length of each candlestick period, in minutes. Must be one minute, one hour, or one day. | [required] |

### Return type

[**models::GetMarketCandlesticksResponse**](GetMarketCandlesticksResponse.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_market_orderbook

> models::GetMarketOrderbookResponse get_market_orderbook(ticker, depth)
GetMarketOrderbook

Endpoint for getting the orderbook for a market.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticker** | **String** | Market ticker. | [required] |
**depth** | Option<**i32**> | Depth specifies the maximum number of orderbook price levels you want to see for either side. Only the highest (most relevant) price level are kept. |  |

### Return type

[**models::GetMarketOrderbookResponse**](GetMarketOrderbookResponse.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_markets

> models::GetMarketsResponse get_markets(limit, cursor, event_ticker, series_ticker, max_close_ts, min_close_ts, status, tickers)
GetMarkets

Endpoint for listing and discovering markets on Kalshi.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i64**> | Parameter to specify the number of results per page. Defaults to 100. |  |
**cursor** | Option<**String**> | The Cursor represents a pointer to the next page of records in the pagination. So this optional parameter, when filled, should be filled with the cursor string returned in a previous request to this end-point. Filling this would basically tell the api to get the next page containing the number of records passed on the limit parameter. On the other side not filling it tells the api you want to get the first page for another query. The cursor does not store any filters, so if any filter parameters like tickers, max_ts or min_ts were passed in the original query they must be passed again. |  |
**event_ticker** | Option<**String**> | Event ticker to retrieve markets for. |  |
**series_ticker** | Option<**String**> | Series ticker to retrieve contracts for. |  |
**max_close_ts** | Option<**i64**> | Restricts the markets to those that are closing in or before this timestamp. |  |
**min_close_ts** | Option<**i64**> | Restricts the markets to those that are closing in or after this timestamp. |  |
**status** | Option<**String**> | Restricts the markets to those with certain statuses, as a comma separated list. The following values are accepted: unopened, open, closed, settled. |  |
**tickers** | Option<**String**> | Restricts the markets to those with certain tickers, as a comma separated list. |  |

### Return type

[**models::GetMarketsResponse**](GetMarketsResponse.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_series

> models::GetSeriesResponse get_series(series_ticker)
GetSeries

Endpoint for getting data about a series by its ticker.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**series_ticker** | **String** | Should be filled with the ticker of the series. | [required] |

### Return type

[**models::GetSeriesResponse**](GetSeriesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_trades

> models::PublicTradesGetResponse get_trades(cursor, limit, ticker, min_ts, max_ts)
GetTrades

Endpoint for getting all trades for all markets.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cursor** | Option<**String**> | The Cursor represents a pointer to the next page of records in the pagination. So this optional parameter, when filled, should be filled with the cursor string returned in a previous request to this end-point. Filling this would basically tell the api to get the next page containing the number of records passed on the limit parameter. On the other side not filling it tells the api you want to get the first page for another query. The cursor does not store any filters, so if any filter parameters like ticker, max_ts or min_ts were passed in the original query they must be passed again. |  |
**limit** | Option<**i32**> | Parameter to specify the number of results per page. Defaults to 100. |  |
**ticker** | Option<**String**> | Parameter to specify a specific market to get trades from. |  |
**min_ts** | Option<**i64**> | Restricts the response to trades after a timestamp. |  |
**max_ts** | Option<**i64**> | Restricts the response to trades before a timestamp. |  |

### Return type

[**models::PublicTradesGetResponse**](PublicTradesGetResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

