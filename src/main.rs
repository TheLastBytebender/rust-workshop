use url::Url;
use serde_json::Value;
use tokio::runtime::Runtime;
use std::collections::HashMap;
use reqwest::{ Client, header::HeaderMap };

pub struct BuildRequest<'a, T> {
    method: &'a str,
    url: &'a str,
    // Payload should be [[str, str]] for "GET"
    // Payload should be HashMap for "POST"
    payload: T, 
    headers: HeaderMap
}

pub struct Executor {
    api_key: String,
    api_secret: String,
    https_endpoint: Url,
    https_alt_endpoint: Url,
    client: Client
}

impl Executor {

    pub fn new(api_key: String, api_secret: String, https_endpoint: Url, https_alt_endpoint: Url) -> Executor {
        Executor {
            api_key: api_key,
            api_secret: api_secret,
            https_endpoint: https_endpoint,
            https_alt_endpoint: https_alt_endpoint,
            client: Client::new()
        }
    }

    pub async fn fetch<'a, T: serde::ser::Serialize> (&self, build_request: BuildRequest<'a, T>) -> Result<Value, reqwest::Error> {
        match  build_request.method {

            "Get" => {
                let resp = self
                    .client
                    .get(build_request.url)
                    .query(&build_request.payload)
                    .headers(build_request.headers)
                    .send()
                    .await?;

                println!("{:?}", resp);
                let resp_object = resp
                    .json::<Value>()
                    .await?;

                Ok(resp_object)
            }

            "Post" => {
                let resp = self
                    .client
                    .post(build_request.url)
                    .json(&build_request.payload)
                    .headers(build_request.headers)
                    .send()
                    .await?;

                let resp_object = resp 
                    .json::<Value>()
                    .await?;

                Ok(resp_object)
            }
            
            _ => todo!()
        }
    }
}

fn main() {

    let rt = Runtime::new().unwrap();

    let executor = Executor::new(
        "hi".to_string(), 
        "hi".to_string(),
        Url::parse("https://api.bybit.com").unwrap(),
        Url::parse("https://api.bybit.com").unwrap()
        );

    let params = [
        ["category", "linear"],
        ["symbol", "BTCUSDT"],
        ["interval", "1"]
        ];

    let url ="https://api.bybit.com/v5/market/kline";

    let headers = HeaderMap::new();

    let build_request = BuildRequest {
        method: "Get",
        url: url,
        payload: params,
        headers: headers
    };

    rt.block_on(async {
        let anw = executor.fetch(build_request).await;
        
        match anw {
            Ok(anw) => println!("{:?}", anw),
            Err(e) => println!("{:?}", e),
        }
    });

}