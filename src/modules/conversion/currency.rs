
extern crate hyper;
extern crate serde;
extern crate serde_json;

use hyper::Request;
use hyper::http::request::Builder;

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
    access_key : String,
    base_url   : String
}

pub struct Symbol {
    code: String,
    name: String
}

use serde::de;
// use serde_json;

fn deserialize<T>(req: Request<Vec<u8>>) -> serde_json::Result<Request<T>>
    where for<'de> T: de::Deserialize<'de>,
{
    let (parts, body) = req.into_parts();
    let body = serde_json::from_slice(&body)?;
    Ok(Request::from_parts(parts, body))
}

impl Fixerio {
    pub fn new(access_key : String) -> Self {
        Self {
            access_key: access_key,
            base_url: String::from("https://data.fixer.io/api"),
        }
    }

    fn get_base_builder(&self, endpoint : FixerioEndpoint) -> Builder {
        let mut uri = String::from(self.base_url.as_str());
        uri.push_str("/");
        uri.push_str(endpoint.get_name().as_str());

        Request::builder()
        .method("GET")
        .uri(uri.as_str())
        .header("access_key", self.access_key.as_str())
    }

    pub fn get_symbols(&self) -> Vec<Symbol> {
        let request = self.get_base_builder(FixerioEndpoint::SYMBOLS)
        .body(())
        .unwrap();

        deserialize(request);

        Vec::new()
    }



}



