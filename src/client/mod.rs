use serde::Serialize;
use reqwest::{Response, Error, RequestBuilder};

use crate::models::{
    LoginRequest,
    LoginResponse,
    GetSeriesResponse,
    GetEventsResponse,
    GetEventResponse,
    GetMarketResponse,
};


const LIVE_BASE_URL: &str = "https://trading-api.kalshi.com/trade-api/v2";
const DEMO_BASE_URL: &str = "https://demo-api.kalshi.co/trade-api/v2";


pub enum Env {
    Live,
    Demo,
}


pub struct Client {
    base_url: String,
    token: Option<String>,
    client: reqwest::Client,
}

impl Client {
    pub fn new(env: Env) -> Self {
        let base_url = match env {
            Env::Live => LIVE_BASE_URL,
            Env::Demo => DEMO_BASE_URL,
        }.to_string();
    
        Self {
            base_url,
            token: None,
            client: reqwest::Client::new(),
        }
    }

    pub async fn login(&mut self, email: &str, password: &str) -> Result<LoginResponse, Error> {
        let req_body = LoginRequest {
            email: email.to_string(),
            password: password.to_string(),
        };

        let res_body = self.build_post_req("/login", &req_body).send().await?.json::<LoginResponse>().await?;

        self.token = res_body.member_id.map(|id| format!("{} {}", id, res_body.token));

        Ok(res_body)
    }

    pub async fn get_series(&mut self, ticker: &str) -> Result<GetSeriesResponse, Error> {
        self.build_get_req(&format!("/series/{ticker}")).send().await?.json::<GetSeriesResponse>().await
    }

    pub async fn get_events(
        &mut self,
        limit: Option<i64>,
        cursor: Option<&str>,
        status: Option<&str>,
        series_ticker: Option<&str>,
        with_nested_markets: Option<bool>,
    ) -> Result<GetEventsResponse, Error> {
        let query = [
            limit.map(|v| ("limit", v.to_string())),
            cursor.map(|v| ("cursor", v.to_string())),
            status.map(|v| ("status", v.to_string())),
            series_ticker.map(|v| ("series_ticker", v.to_string())),
            with_nested_markets.map(|v| ("with_nested_markets", v.to_string())),
        ];

        self.apply_query(self.build_get_req("/events"), &query).send().await?.json::<GetEventsResponse>().await
    }

    pub async fn get_event(&mut self, ticker: &str, with_nested_markets: Option<bool>) -> Result<GetEventResponse, Error> {
        let query = [
            with_nested_markets.map(|v| ("with_nested_markets", v.to_string())),
        ];

        self.apply_query(self.build_get_req(&format!("/events/{ticker}")), &query).send().await?.json::<GetEventResponse>().await
    }

    pub async fn get_market(&mut self, ticker: &str) -> Result<GetMarketResponse, Error> {
        self.build_get_req(&format!("/markets/{ticker}")).send().await?.json::<GetMarketResponse>().await
    }

    fn build_get_req(&self, path: &str) -> RequestBuilder {
        self.apply_auth(self.client.get(&self.construct_url(path)))
    }

    fn build_post_req<T: Serialize>(&self, path: &str, body: &T) -> RequestBuilder {
        self.apply_auth(self.client.post(&self.construct_url(path))).json(body)
    }

    fn apply_auth(&self, req: RequestBuilder) -> RequestBuilder {
        if let Some(ref token) = self.token {
            req.header(reqwest::header::AUTHORIZATION, token)
        } else {
            req
        }
    }

    fn apply_query(&self, req: RequestBuilder, query: &[Option<(&str, String)>]) -> RequestBuilder {
        query.iter().flatten().fold(req, |req, (key, value)| {
            req.query(&[(key, value)])
        })
    }

    fn construct_url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    async fn new_client() -> Client {
        let mut client = Client::new(Env::Demo);
        client.login(&env::var("KALSHI_EMAIL").unwrap(), &env::var("KALSHI_PASSWORD").unwrap()).await.unwrap();
        client
    }

    #[tokio::test]
    async fn login_error() {
        let mut client = Client::new(Env::Demo);
        client.login("abc", "123").await.unwrap_err();
    }

    #[tokio::test]
    async fn get_series() {
        let mut client = new_client().await;
        let res = client.get_series("INX").await.unwrap();
        println!("{res:#?}");
    }

    #[tokio::test]
    async fn get_events() {
        let mut client = new_client().await;
        let res = client.get_events(None, None, None, None, None).await.unwrap();
        println!("{res:#?}");
    }

    #[tokio::test]
    async fn get_event() {
        let mut client = new_client().await;
        let res = client.get_event("INXU-24AUG16", Some(true)).await.unwrap();
        println!("{res:#?}");
    }

    #[tokio::test]
    async fn get_market() {
        let mut client = new_client().await;
        let res = client.get_market("INXU-24AUG16-T5543.22").await.unwrap();
        println!("{res:#?}");
    }
}
