use crate::CONFIG;
use failure::Error;
use serde_json::Value;

fn req_configure(req: &mut ureq::Request) {
    req.set("Content-Type", "application/json");

    if let Some(timeout) = CONFIG.read().timeout {
        req.timeout_connect(timeout);
        req.timeout_read(timeout);
        req.timeout_write(timeout);
    }
}

pub fn request(path: &str, request_body: Option<Value>) -> Result<String, Error> {
    let url = format!("{}{}", CONFIG.read().base_url, path);
    debug!("Request URL: {}", url);
    let mut request = ureq::post(&url);
    req_configure(&mut request);

    let response = match request_body {
        Some(json) => {
            debug!("Request body: {:?}", json);
            request.send_json(json)
        }
        _ => {
            debug!("Empty request body...");
            request.call()
        }
    };

    if response.ok() {
        Ok(response.into_string()?)
    } else {
        Err(format_err!(
            "Request failed with status code {}",
            response.status()
        ))
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
        let server = test_server::new(0, HttpResponse::BadRequest)?;
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
        let server = test_server::new(0, HttpResponse::BadRequest)?;
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
        let server = test_server::new(0, HttpResponse::BadRequest)?;
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
