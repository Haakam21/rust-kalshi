use serde::Serialize;
use reqwest::{Response, Error};

use crate::models::{
    LoginRequest,
    LoginResponse,
    GetMarketsResponse,
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

        let res = self.post("/login", &req_body).await?;

        let res_body = res.json::<LoginResponse>().await?;
        self.token = Some(format!("{} {}", res_body.member_id.unwrap(), res_body.token));

        Ok(res_body)
    }

    pub async fn get_markets(&mut self) -> Result<GetMarketsResponse, Error> {
        let res = self.get("/markets").await?;

        res.json::<GetMarketsResponse>().await
    }

    pub async fn get_market(&mut self, ticker: &str) -> Result<GetMarketResponse, Error> {
        let res = self.get(&format!("/markets/{ticker}")).await?;

        res.json::<GetMarketResponse>().await
    }

    async fn get(&self, path: &str) -> Result<Response, Error> {
        self.client.get(self.base_url.clone() + path).header(reqwest::header::AUTHORIZATION, reqwest::header::HeaderValue::from_str(self.token.as_ref().unwrap()).unwrap()).send().await
    }

    async fn post<T: Serialize>(&self, path: &str, body: &T) -> Result<Response, Error> {
        self.client.post(self.base_url.clone() + path).json(body).send().await
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
    async fn get_markets() {
        let mut client = new_client().await;
        let res = client.get_markets().await.unwrap();
        println!("{res:#?}");
    }

    #[tokio::test]
    async fn get_market() {
        let mut client = new_client().await;
        let res = client.get_market("INXU-24AUG16-T5543.22").await.unwrap();
        println!("{res:#?}");
    }
}
