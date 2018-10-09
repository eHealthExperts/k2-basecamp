use super::settings::Settings;
use reqwest;
use std::collections::HashMap;
use std::io::Error;
use std::str;
use std::time::Duration;

pub struct Response {
    pub status: u16,
    pub body: String,
}

pub fn request(path: &str, request_body: Option<HashMap<&str, String>>) -> Result<Response, Error> {
    let mut client_builder = reqwest::Client::builder();

    if let Some(seconds) = Settings::timeout() {
        client_builder = client_builder.timeout(Duration::from_secs(seconds));
    }

    let client = client_builder
        .build()
        .expect("Failed to create HTTP client");

    let mut request_builder = client.post(&uri(path));

    if let Some(json) = request_body {
        debug!("Request body: {:?}", json);
        request_builder = request_builder.json(&json);
    } else {
        debug!("Empty request body...");
    }

    let mut status: u16 = 0;
    let mut body = String::new();

    if let Ok(mut response) = request_builder.send() {
        status = response.status().into();
        body = response.text().expect("Failed to get response body")
    }

    debug!("Response status: {}", status);
    debug!("Response body: {}", body);
    Ok(Response { status, body })
}

fn uri(path: &str) -> String {
    let mut addr = Settings::base_url();
    addr.push_str(path);
    debug!("Request URL: {}", addr);
    addr
}

#[cfg(test)]
mod tests {

    use super::{request, Response};
    use rand::distributions::Alphanumeric;
    use rand::{self, Rng};
    use serde_json::{self, Value};
    use std::collections::HashMap;
    use std::env;
    use test_server::{self, HttpResponse};

    #[test]
    fn request_with_body_is_content_type_json() {
        let server = test_server::new(0, |_| HttpResponse::BadRequest().into());
        env::set_var("K2_BASE_URL", server.url());

        let mut body = HashMap::new();
        body.insert("body", create_rand_string(100));

        let _ = request("", Some(body));
        let request = server.requests.next().unwrap();

        assert_eq!(
            Some(&String::from("application/json")),
            request.headers.get("content-type")
        );
    }

    #[test]
    fn send_request_body_if_given() {
        let server = test_server::new(0, |_| HttpResponse::BadRequest().into());
        env::set_var("K2_BASE_URL", server.url());

        let mut body = HashMap::new();
        body.insert("body", create_rand_string(100));

        let _ = request("", Some(body.clone()));
        let request = server.requests.next().unwrap();
        let json: Value = serde_json::from_str(&request.body).unwrap();

        assert_eq!(json!(body), json);
    }

    #[test]
    fn if_no_json_is_given_send_empty_request_body() {
        let server = test_server::new(0, |_| HttpResponse::BadRequest().into());
        env::set_var("K2_BASE_URL", server.url());

        let _ = request("", None);
        let request = server.requests.next().unwrap();

        assert!(request.body.is_empty());
    }

    #[test]
    fn response_contains_status_and_body() {
        let server = test_server::new(0, |_| {
            HttpResponse::InternalServerError().body("hello world")
        });
        env::set_var("K2_BASE_URL", server.url());
        let response: Response = request("", None).unwrap();

        assert_eq!(response.status, 500);
        assert_eq!(response.body, "hello world");
    }

    fn create_rand_string(size: usize) -> String {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(size)
            .collect::<String>()
    }
}
