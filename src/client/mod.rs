use serde::Serialize;
use reqwest::{Response, Error, RequestBuilder};

use crate::models::{
    LoginRequest,
    LoginResponse,
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
        Self {
            base_url: match env {
                Env::Live => LIVE_BASE_URL.to_string(),
                Env::Demo => DEMO_BASE_URL.to_string(),
            },
            token: None,
            client: reqwest::Client::new(),
        }
    }

    pub async fn login(&mut self, email: &str, password: &str) -> Result<LoginResponse, Error> {
        let req_body = LoginRequest {
            email: email.to_string(),
            password: password.to_string(),
        };

        let res = self.build_post("/login", &req_body).send().await?;

        let res_body = res.json::<LoginResponse>().await?;

        self.token = Some(format!("{} {}", res_body.member_id.unwrap(), res_body.token));

        Ok(res_body)
    }

    pub async fn get_event(&mut self, ticker: &str, with_nested_markets: Option<bool>) -> Result<GetEventResponse, Error> {
        let mut req = self.build_get(&format!("/events/{ticker}"));

        if let Some(value) = with_nested_markets {
            req = req.query(&[("with_nested_markets", value)]);
        }

        let res = req.send().await?;

        res.json::<GetEventResponse>().await
    }

    pub async fn get_market(&mut self, ticker: &str) -> Result<GetMarketResponse, Error> {
        let res = self.build_get(&format!("/markets/{ticker}")).send().await?;

        res.json::<GetMarketResponse>().await
    }

    fn build_get(&self, path: &str) -> RequestBuilder {
        self.build_auth(self.client.get(self.build_url(path)))
    }

    fn build_post<T: Serialize>(&self, path: &str, body: &T) -> RequestBuilder {
        self.build_auth(self.client.post(self.build_url(path))).json(body)
    }

    fn build_auth(&self, req: RequestBuilder) -> RequestBuilder {
        if let Some(token) = &self.token {
            return req.header(reqwest::header::AUTHORIZATION, token)
        }
        req
    }

    fn build_url(&self, path: &str) -> String {
        self.base_url.clone() + path
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
