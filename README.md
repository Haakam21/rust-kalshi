# Rust API client for kalshi

This documentation describes Kalshi's trading API (known as Trade API v2).
By using this API, you agree to Kalshi's Developer Agreement (https://kalshi.com/developer-agreement).


## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 2.0.0
- Package version: 2.0.0
- Generator version: 7.7.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `kalshi` and add the following to `Cargo.toml` under `[dependencies]`:

```
kalshi = { path = "./kalshi" }
```

## Documentation for API Endpoints

All URIs are relative to *https://trading-api.kalshi.com/trade-api/v2*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*AuthApi* | [**login**](docs/AuthApi.md#login) | **POST** /login | Login
*AuthApi* | [**logout**](docs/AuthApi.md#logout) | **POST** /logout | Logout
*ExchangeApi* | [**get_exchange_announcements**](docs/ExchangeApi.md#get_exchange_announcements) | **GET** /exchange/status | GetExchangeAnnouncements
*ExchangeApi* | [**get_exchange_schedule**](docs/ExchangeApi.md#get_exchange_schedule) | **GET** /exchange/schedule | GetExchangeSchedule
*MarketApi* | [**get_event**](docs/MarketApi.md#get_event) | **GET** /events/{event_ticker} | GetEvent
*MarketApi* | [**get_events**](docs/MarketApi.md#get_events) | **GET** /events | GetEvents
*MarketApi* | [**get_market**](docs/MarketApi.md#get_market) | **GET** /markets/{ticker} | GetMarket
*MarketApi* | [**get_market_candlesticks**](docs/MarketApi.md#get_market_candlesticks) | **GET** /series/{series_ticker}/markets/{ticker}/candlesticks | GetMarketCandlesticks
*MarketApi* | [**get_market_orderbook**](docs/MarketApi.md#get_market_orderbook) | **GET** /markets/{ticker}/orderbook | GetMarketOrderbook
*MarketApi* | [**get_markets**](docs/MarketApi.md#get_markets) | **GET** /markets | GetMarkets
*MarketApi* | [**get_series**](docs/MarketApi.md#get_series) | **GET** /series/{series_ticker} | GetSeries
*MarketApi* | [**get_trades**](docs/MarketApi.md#get_trades) | **GET** /markets/trades | GetTrades
*PortfolioApi* | [**amend_order**](docs/PortfolioApi.md#amend_order) | **POST** /portfolio/orders/{order_id}/amend | AmendOrder
*PortfolioApi* | [**batch_cancel_orders**](docs/PortfolioApi.md#batch_cancel_orders) | **DELETE** /portfolio/orders/batched | BatchCancelOrders
*PortfolioApi* | [**batch_create_orders**](docs/PortfolioApi.md#batch_create_orders) | **POST** /portfolio/orders/batched | BatchCreateOrders
*PortfolioApi* | [**cancel_order**](docs/PortfolioApi.md#cancel_order) | **DELETE** /portfolio/orders/{order_id} | CancelOrder
*PortfolioApi* | [**create_order**](docs/PortfolioApi.md#create_order) | **POST** /portfolio/orders | CreateOrder
*PortfolioApi* | [**decrease_order**](docs/PortfolioApi.md#decrease_order) | **POST** /portfolio/orders/{order_id}/decrease | DecreaseOrder
*PortfolioApi* | [**get_balance**](docs/PortfolioApi.md#get_balance) | **GET** /portfolio/balance | GetBalance
*PortfolioApi* | [**get_fills**](docs/PortfolioApi.md#get_fills) | **GET** /portfolio/fills | GetFills
*PortfolioApi* | [**get_order**](docs/PortfolioApi.md#get_order) | **GET** /portfolio/orders/{order_id} | GetOrder
*PortfolioApi* | [**get_orders**](docs/PortfolioApi.md#get_orders) | **GET** /portfolio/orders | GetOrders
*PortfolioApi* | [**get_portfolio_resting_order_total_value**](docs/PortfolioApi.md#get_portfolio_resting_order_total_value) | **GET** /portfolio/summary/total_resting_order_value | GetPortfolioRestingOrderTotalValue
*PortfolioApi* | [**get_portfolio_settlements**](docs/PortfolioApi.md#get_portfolio_settlements) | **GET** /portfolio/settlements | GetPortfolioSettlements
*PortfolioApi* | [**get_positions**](docs/PortfolioApi.md#get_positions) | **GET** /portfolio/positions | GetPositions


## Documentation For Models

 - [AmendOrderRequest](docs/AmendOrderRequest.md)
 - [AmendOrderResponse](docs/AmendOrderResponse.md)
 - [Announcement](docs/Announcement.md)
 - [BatchCancelOrdersIndividualResponse](docs/BatchCancelOrdersIndividualResponse.md)
 - [BatchCancelOrdersRequest](docs/BatchCancelOrdersRequest.md)
 - [BatchCancelOrdersResponse](docs/BatchCancelOrdersResponse.md)
 - [BatchCreateOrdersIndividualResponse](docs/BatchCreateOrdersIndividualResponse.md)
 - [BatchCreateOrdersRequest](docs/BatchCreateOrdersRequest.md)
 - [BatchCreateOrdersResponse](docs/BatchCreateOrdersResponse.md)
 - [BidAskDistribution](docs/BidAskDistribution.md)
 - [CancelOrderResponse](docs/CancelOrderResponse.md)
 - [CreateOrderRequest](docs/CreateOrderRequest.md)
 - [CreateOrderResponse](docs/CreateOrderResponse.md)
 - [DailySchedule](docs/DailySchedule.md)
 - [DecreaseOrderRequest](docs/DecreaseOrderRequest.md)
 - [DecreaseOrderResponse](docs/DecreaseOrderResponse.md)
 - [EventData](docs/EventData.md)
 - [EventPosition](docs/EventPosition.md)
 - [ExchangeStatus](docs/ExchangeStatus.md)
 - [Fill](docs/Fill.md)
 - [GetBalanceResponse](docs/GetBalanceResponse.md)
 - [GetEventResponse](docs/GetEventResponse.md)
 - [GetEventsResponse](docs/GetEventsResponse.md)
 - [GetExchangeAnnouncementsResponse](docs/GetExchangeAnnouncementsResponse.md)
 - [GetExchangeScheduleResponse](docs/GetExchangeScheduleResponse.md)
 - [GetFillsResponse](docs/GetFillsResponse.md)
 - [GetMarketCandlesticksResponse](docs/GetMarketCandlesticksResponse.md)
 - [GetMarketOrderbookResponse](docs/GetMarketOrderbookResponse.md)
 - [GetMarketResponse](docs/GetMarketResponse.md)
 - [GetMarketsResponse](docs/GetMarketsResponse.md)
 - [GetOrderResponse](docs/GetOrderResponse.md)
 - [GetOrdersResponse](docs/GetOrdersResponse.md)
 - [GetPortfolioSettlementsResponse](docs/GetPortfolioSettlementsResponse.md)
 - [GetPositionsResponse](docs/GetPositionsResponse.md)
 - [GetSeriesResponse](docs/GetSeriesResponse.md)
 - [GetUserRestingOrderTotalValueResponse](docs/GetUserRestingOrderTotalValueResponse.md)
 - [JsonError](docs/JsonError.md)
 - [LoginRequest](docs/LoginRequest.md)
 - [LoginResponse](docs/LoginResponse.md)
 - [MaintenanceWindow](docs/MaintenanceWindow.md)
 - [Market](docs/Market.md)
 - [MarketCandlestick](docs/MarketCandlestick.md)
 - [MarketPosition](docs/MarketPosition.md)
 - [Order](docs/Order.md)
 - [OrderBook](docs/OrderBook.md)
 - [OrderConfirmation](docs/OrderConfirmation.md)
 - [PriceDistribution](docs/PriceDistribution.md)
 - [PublicTrade](docs/PublicTrade.md)
 - [PublicTradesGetResponse](docs/PublicTradesGetResponse.md)
 - [Schedule](docs/Schedule.md)
 - [Series](docs/Series.md)
 - [Settlement](docs/Settlement.md)
 - [SettlementSource](docs/SettlementSource.md)
 - [WeeklySchedule](docs/WeeklySchedule.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author

support@kalshi.com

