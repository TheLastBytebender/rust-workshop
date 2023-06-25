use url::Url;
use reqwest::Client;
use serde_json::Value;

// This Executor will mainly be tailored towards Bybit Api

struct BuildRequest {
	url: Url,
	payload: Value,
	headers: Value
}

struct Executor {
	api_key: String,
	api_secret: String,
	https_endpoint: Url,
	https_alt_endpoint: Url,
	client: Client
}

impl Executor {

	fn new(api_key: String, api_secret: String, https_endpoint: Url, https_alt_endpoint: Url) -> Executor {
		Executor {
			api_key: api_key,
			api_secret: api_secret,
			https_endpoint: https_endpoint,
			https_alt_endpoint: https_alt_endpoint,
			client: Client::new()
		}
	}

	fn fetch(&self, method: &str, build_request: BuildRequest) -> Value {
		todo!()
	}

}

