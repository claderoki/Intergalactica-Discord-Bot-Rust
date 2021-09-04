use serde::de::DeserializeOwned;
use serenity::async_trait;
extern crate reqwest;

use std::collections::HashMap;

pub trait Api {
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
