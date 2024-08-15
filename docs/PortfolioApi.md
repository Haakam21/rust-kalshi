# \PortfolioApi

All URIs are relative to *https://trading-api.kalshi.com/trade-api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**amend_order**](PortfolioApi.md#amend_order) | **POST** /portfolio/orders/{order_id}/amend | AmendOrder
[**batch_cancel_orders**](PortfolioApi.md#batch_cancel_orders) | **DELETE** /portfolio/orders/batched | BatchCancelOrders
[**batch_create_orders**](PortfolioApi.md#batch_create_orders) | **POST** /portfolio/orders/batched | BatchCreateOrders
[**cancel_order**](PortfolioApi.md#cancel_order) | **DELETE** /portfolio/orders/{order_id} | CancelOrder
[**create_order**](PortfolioApi.md#create_order) | **POST** /portfolio/orders | CreateOrder
[**decrease_order**](PortfolioApi.md#decrease_order) | **POST** /portfolio/orders/{order_id}/decrease | DecreaseOrder
[**get_balance**](PortfolioApi.md#get_balance) | **GET** /portfolio/balance | GetBalance
[**get_fills**](PortfolioApi.md#get_fills) | **GET** /portfolio/fills | GetFills
[**get_order**](PortfolioApi.md#get_order) | **GET** /portfolio/orders/{order_id} | GetOrder
[**get_orders**](PortfolioApi.md#get_orders) | **GET** /portfolio/orders | GetOrders
[**get_portfolio_resting_order_total_value**](PortfolioApi.md#get_portfolio_resting_order_total_value) | **GET** /portfolio/summary/total_resting_order_value | GetPortfolioRestingOrderTotalValue
[**get_portfolio_settlements**](PortfolioApi.md#get_portfolio_settlements) | **GET** /portfolio/settlements | GetPortfolioSettlements
[**get_positions**](PortfolioApi.md#get_positions) | **GET** /portfolio/positions | GetPositions



## amend_order

> models::AmendOrderResponse amend_order(order_id, amend_order_request)
AmendOrder

Endpoint for amending the max number of fillable contracts and/or price in an existing order.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_id** | **uuid::Uuid** | ID of the order to be amended. | [required] |
**amend_order_request** | [**AmendOrderRequest**](AmendOrderRequest.md) | Order data | [required] |

### Return type

[**models::AmendOrderResponse**](AmendOrderResponse.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## batch_cancel_orders

> models::BatchCancelOrdersResponse batch_cancel_orders(batch_cancel_orders_request)
BatchCancelOrders

Endpoint for cancelling up to 20 orders at once. Available to members with advanced access only.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**batch_cancel_orders_request** | [**BatchCancelOrdersRequest**](BatchCancelOrdersRequest.md) | Batch orders cancel input data. | [required] |

### Return type

[**models::BatchCancelOrdersResponse**](BatchCancelOrdersResponse.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## batch_create_orders

> models::BatchCreateOrdersResponse batch_create_orders(batch_create_orders_request)
BatchCreateOrders

Endpoint for submitting a batch of orders.  Each order in the batch is counted against the total rate limit for order operations. Consequently, the size of the batch is capped by the current per-second rate-limit configuration applicable to the user.  At the moment of writing, the limit is 20 orders per batch. Available to members with advanced access only.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**batch_create_orders_request** | [**BatchCreateOrdersRequest**](BatchCreateOrdersRequest.md) | Batch order create input data. | [required] |

### Return type

[**models::BatchCreateOrdersResponse**](BatchCreateOrdersResponse.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cancel_order

> models::CancelOrderResponse cancel_order(order_id)
CancelOrder

Endpoint for canceling orders.  The value for the orderId should match the id field of the order you want to decrease. Commonly, DELETE-type endpoints return 204 status with no body content on success. But we can't completely delete the order, as it may be partially filled already. Instead, the DeleteOrder endpoint reduce the order completely, essentially zeroing the remaining resting contracts on it. The zeroed order is returned on the response payload as a form of validation for the client.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_id** | **uuid::Uuid** | Order_id input for the current order. | [required] |

### Return type

[**models::CancelOrderResponse**](CancelOrderResponse.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_order

> models::CreateOrderResponse create_order(create_order_request)
CreateOrder

Endpoint for submitting orders in a market.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_order_request** | [**CreateOrderRequest**](CreateOrderRequest.md) | Order create input data | [required] |

### Return type

[**models::CreateOrderResponse**](CreateOrderResponse.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## decrease_order

> models::DecreaseOrderResponse decrease_order(order_id, decrease_order_request)
DecreaseOrder

Endpoint for decreasing the number of contracts in an existing order. This is the only kind of edit available on order quantity. Cancelling an order is equivalent to decreasing an order amount to zero.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_id** | **uuid::Uuid** | ID of the order to be decreased. | [required] |
**decrease_order_request** | [**DecreaseOrderRequest**](DecreaseOrderRequest.md) | Order data | [required] |

### Return type

[**models::DecreaseOrderResponse**](DecreaseOrderResponse.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_balance

> models::GetBalanceResponse get_balance()
GetBalance

Endpoint for getting the balance of the logged-in member.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetBalanceResponse**](GetBalanceResponse.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_fills

> models::GetFillsResponse get_fills(ticker, order_id, min_ts, max_ts, limit, cursor)
GetFills

Endpoint for getting all fills for the logged-in member.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticker** | Option<**String**> | Restricts the response to trades in a specific market. |  |
**order_id** | Option<**uuid::Uuid**> | Restricts the response to trades related to a specific order. |  |
**min_ts** | Option<**i64**> | Restricts the response to trades after a timestamp. |  |
**max_ts** | Option<**i64**> | Restricts the response to trades before a timestamp. |  |
**limit** | Option<**i32**> | Parameter to specify the number of results per page. Defaults to 100. |  |
**cursor** | Option<**String**> | The Cursor represents a pointer to the next page of records in the pagination. So this optional parameter, when filled, should be filled with the cursor string returned in a previous request to this end-point. Filling this would basically tell the api to get the next page containing the number of records passed on the limit parameter. On the other side not filling it tells the api you want to get the first page for another query. The cursor does not store any filters, so if any filter parameters like ticker, max_ts or min_ts were passed in the original query they must be passed again. |  |

### Return type

[**models::GetFillsResponse**](GetFillsResponse.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_order

> models::GetOrderResponse get_order(order_id)
GetOrder

Endpoint for getting a single order.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_id** | **uuid::Uuid** | Order_id input for the current order. | [required] |

### Return type

[**models::GetOrderResponse**](GetOrderResponse.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_orders

> models::GetOrdersResponse get_orders(ticker, event_ticker, min_ts, max_ts, status, cursor, limit)
GetOrders

Endpoint for getting all orders for the logged-in member.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticker** | Option<**String**> | Restricts the response to orders in a single market. |  |
**event_ticker** | Option<**String**> | Restricts the response to orders in a single event. |  |
**min_ts** | Option<**i64**> | Restricts the response to orders after a timestamp, formatted as a Unix Timestamp. |  |
**max_ts** | Option<**i64**> | Restricts the response to orders before a timestamp, formatted as a Unix Timestamp. |  |
**status** | Option<**String**> | Restricts the response to orders that have a certain status: resting, canceled, or executed. |  |
**cursor** | Option<**String**> | The Cursor represents a pointer to the next page of records in the pagination. So this optional parameter, when filled, should be filled with the cursor string returned in a previous request to this end-point. Filling this would basically tell the api to get the next page containing the number of records passed on the limit parameter. On the other side not filling it tells the api you want to get the first page for another query. The cursor does not store any filters, so if any filter parameters like ticker, max_ts or min_ts were passed in the original query they must be passed again. |  |
**limit** | Option<**i32**> | Parameter to specify the number of results per page. Defaults to 100. |  |

### Return type

[**models::GetOrdersResponse**](GetOrdersResponse.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_portfolio_resting_order_total_value

> models::GetUserRestingOrderTotalValueResponse get_portfolio_resting_order_total_value()
GetPortfolioRestingOrderTotalValue

Endpoint for getting the total value, in cents, of resting orders. This endpoint is only intended for use by FCM members (rare). Note: If you're uncertain about this endpoint, it likely does not apply to you.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetUserRestingOrderTotalValueResponse**](GetUserRestingOrderTotalValueResponse.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_portfolio_settlements

> models::GetPortfolioSettlementsResponse get_portfolio_settlements(limit, min_ts, max_ts, cursor)
GetPortfolioSettlements

Endpoint for getting the logged-in member's settlements historical track.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i64**> | Parameter to specify the number of results per page. Defaults to 100. |  |
**min_ts** | Option<**i64**> | Restricts the response to settlements after a timestamp. |  |
**max_ts** | Option<**i64**> | Restricts the response to settlements before a timestamp. |  |
**cursor** | Option<**String**> | The Cursor represents a pointer to the next page of records in the pagination. So this optional parameter, when filled, should be filled with the cursor string returned in a previous request to this end-point. Filling this would basically tell the api to get the next page containing the number of records passed on the limit parameter. On the other side not filling it tells the api you want to get the first page for another query. |  |

### Return type

[**models::GetPortfolioSettlementsResponse**](GetPortfolioSettlementsResponse.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_positions

> models::GetPositionsResponse get_positions(cursor, limit, count_filter, settlement_status, ticker, event_ticker)
GetPositions

Endpoint for getting all market positions for the logged-in member.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cursor** | Option<**String**> | The Cursor represents a pointer to the next page of records in the pagination. So this optional parameter, when filled, should be filled with the cursor string returned in a previous request to this end-point. Filling this would basically tell the api to get the next page containing the number of records passed on the limit parameter. On the other side not filling it tells the api you want to get the first page for another query. The cursor does not store any filters, so if any filter parameters like settlement_status, ticker, or event_ticker were passed in the original query they must be passed again. |  |
**limit** | Option<**i32**> | Parameter to specify the number of results per page. Defaults to 100. |  |
**count_filter** | Option<**String**> | Restricts the positions to those with any of following fields with non-zero values, as a comma separated list. The following values are accepted: position, total_traded, resting_order_count |  |
**settlement_status** | Option<**String**> | Settlement status of the markets to return. Defaults to unsettled. all SettlementStatusAll settled SettlementStatusSettled unsettled SettlementStatusUnsettled |  |
**ticker** | Option<**String**> | Ticker of desired positions. |  |
**event_ticker** | Option<**String**> | Event ticker of desired positions. |  |

### Return type

[**models::GetPositionsResponse**](GetPositionsResponse.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

