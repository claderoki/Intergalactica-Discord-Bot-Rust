extern crate reqwest;
extern crate serde;

use super::models::{RatesResponse, SymbolsResponse};
use serde::de::DeserializeOwned;

trait ParamBuilder {

}

struct RatesParamBuilder {

}

impl ParamBuilder for RatesParamBuilder {
    
}


enum FixerioEndpoint {
    SYMBOLS,
    LATEST,
}

impl FixerioEndpoint {
    pub fn get_name(&self) -> String {
        match self {
            Self::SYMBOLS => String::from("symbols"),
            Self::LATEST => String::from("latest"),
        }
    }
}

pub struct Fixerio {
    access_key: String,
    base_url: String,
}

impl Fixerio {
    pub fn new(access_key: String) -> Self {
        Self {
            access_key: access_key,
            base_url: String::from("http://data.fixer.io/api"),
        }
    }

    fn get_base_uri(&self, endpoint: FixerioEndpoint) -> String {
        let mut uri = String::from(self.base_url.as_str());
        uri.push_str("/");
        uri.push_str(endpoint.get_name().as_str());
        uri.push_str("?");
        uri.push_str("access_key=");
        uri.push_str(self.access_key.as_str());
        uri
    }

    async fn call<T>(&self, uri: String) -> Result<T, &'static str>
    where
        T: DeserializeOwned,
    {
        let response = match reqwest::get(uri.as_str()).await {
            Ok(response) => response,
            Err(e) => {
                println!("{:?}", e);
                return Err("something went wrong");
            }
        };

        match response.json::<T>().await {
            Ok(data) => return Ok(data),
            Err(e) => {
                println!("{:?}", e);
                return Err("something went wrong");
            }
        };
    }

    pub async fn get_symbols(&self) -> Result<SymbolsResponse, &'static str> {
        let uri = self.get_base_uri(FixerioEndpoint::SYMBOLS);
        let data = self.call::<SymbolsResponse>(uri).await?;
        Ok(data)
    }

    pub async fn get_rates(&self) -> Result<RatesResponse, &'static str> {
        let uri = self.get_base_uri(FixerioEndpoint::LATEST);
        let data = self.call::<RatesResponse>(uri).await?;
        Ok(data)
    }
}
