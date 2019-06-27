use crate::CONFIG;
use failure::Error;
use reqwest::{self, Client, StatusCode};
use serde_json::Value;
use std::time::Duration;

lazy_static! {
    static ref CLIENT: Client = Client::builder()
        .timeout(match CONFIG.read().timeout {
            Some(timeout) => Some(Duration::from_secs(timeout)),
            None => None,
        })
        .build()
        .expect("Failed to create HTTP client!");
}

pub fn request(path: &str, request_body: Option<Value>) -> Result<String, Error> {
    let url = format!("{}{}", CONFIG.read().base_url, path);
    debug!("Request URL: {}", url);
    let mut request_builder = CLIENT.post(&url);

    if let Some(json) = request_body {
        debug!("Request body: {:?}", json);
        request_builder = request_builder.json(&json);
    } else {
        debug!("Empty request body...");
    }

    let mut response = request_builder.send()?;
    match response.status() {
        StatusCode::OK => Ok(response.text()?),
        s => Err(format_err!("Request failed with status code {}", s)),
    }
}

#[cfg(test)]
mod tests {

    use super::request;
    use crate::{Settings, CONFIG};
    use failure::Error;
    use serde_json::{self, Value};
    use std::env;
    use test_server::{self, helper, HttpResponse};

    #[test]
    fn request_with_body_is_content_type_json() -> Result<(), Error> {
        let server = test_server::new(0, |_| HttpResponse::BadRequest().into())?;
        env::set_var("K2_BASE_URL", server.url());
        init_config();

        let _ = request("", Some(json!({ "body": helper::random_string(100) })));
        let request = server.requests.next().unwrap();

        assert_eq!(
            Some(&String::from("application/json")),
            request.headers.get("content-type")
        );

        env::remove_var("K2_BASE_URL");

        Ok(())
    }

    #[test]
    fn send_request_body_if_given() -> Result<(), Error> {
        let server = test_server::new(0, |_| HttpResponse::BadRequest().into())?;
        env::set_var("K2_BASE_URL", server.url());
        init_config();

        let body = json!({ "body": helper::random_string(100) });

        let _ = request("", Some(body.clone()));
        let request = server.requests.next().unwrap();
        let json: Value = serde_json::from_str(&request.body).unwrap();

        assert_eq!(body, json);

        env::remove_var("K2_BASE_URL");

        Ok(())
    }

    #[test]
    fn if_no_json_is_given_send_empty_request_body() -> Result<(), Error> {
        let server = test_server::new(0, |_| HttpResponse::BadRequest().into())?;
        env::set_var("K2_BASE_URL", server.url());
        init_config();

        let _ = request("", None);
        let request = server.requests.next().unwrap();

        assert!(request.body.is_empty());

        env::remove_var("K2_BASE_URL");

        Ok(())
    }

    fn init_config() {
        let mut config_guard = CONFIG.write();
        *config_guard = Settings::init().unwrap();
        drop(config_guard);
    }
}
