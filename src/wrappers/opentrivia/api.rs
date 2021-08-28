use serde::de::DeserializeOwned;
use serenity::async_trait;
extern crate reqwest;

use std::collections::HashMap;

use super::models::Trivia;

trait Api {
    fn get_base_uri() -> String;
}

pub enum ApiCallMethod {
    Get,
    _Post,
}

#[async_trait]
pub trait ApiCall<D: DeserializeOwned> {
    fn get_uri(&self) -> String;

    async fn call(&self) -> Result<D, String> {
        let uri = {
            if let Some(params) = self.get_query_params() {
                if params.is_empty() {
                    self.get_uri()
                } else {
                    let addition = {
                        let mut messages: Vec<String> = Vec::new();
                        for (key, value) in params.iter() {
                            messages.push(format!("{}={}", key, value));
                        }
                        messages.join("&")
                    };
                    format!("{}?{}", self.get_uri(), addition)
                }
            } else {
                self.get_uri()
            }
        };

        match Self::get_method() {
            ApiCallMethod::Get => {
                let response = reqwest::get(uri.as_str())
                    .await
                    .map_err(|e| format!("{}", e))?;
                response.json::<D>().await.map_err(|e| format!("{}", e))
            }
            _ => Err("Not implemented yet.".into()),
        }
    }

    fn get_method() -> ApiCallMethod {
        ApiCallMethod::Get
    }

    fn get_query_params(&self) -> Option<HashMap<String, String>> {
        None
    }
}

pub struct TriviaCall {
    pub amount: i32,
    // pub category   (25)
    // pub difficult  (easy)
    // pub type       (multiple)
}

impl TriviaCall {
    pub fn new() -> Self {
        Self { amount: 10 }
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct TriviaListResponse {
    pub results: Vec<Trivia>,
}

impl ApiCall<TriviaListResponse> for TriviaCall {
    fn get_uri(&self) -> String {
        format!("{}/api.php", OpenTriviaApi::get_base_uri())
    }

    fn get_query_params(&self) -> Option<HashMap<String, String>> {
        let mut params: HashMap<String, String> = HashMap::new();
        params.insert("amount".into(), self.amount.to_string());
        Some(params)
    }
}

pub struct OpenTriviaApi;
impl Api for OpenTriviaApi {
    fn get_base_uri() -> String {
        "https://opentdb.com".into()
    }
}
