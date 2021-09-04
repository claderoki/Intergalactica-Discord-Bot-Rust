extern crate reqwest;

use std::collections::HashMap;

use super::base_api::ApiCall;
use super::base_api::Api;
use super::models::Trivia;

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
