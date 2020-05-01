use crate::CONFIG;
use failure::Error;
use serde_json::Value;

pub fn request(path: &str, request_body: Option<Value>) -> Result<String, Error> {
    let url = format!("{}{}", CONFIG.read().base_url, path);
    debug!("Request URL: {}", url);
    let mut request = ureq::post(&url);

    request.set("Content-Type", "application/json");

    if let Some(timeout) = CONFIG.read().timeout {
        request.timeout_connect(timeout * 1000);
        request.timeout_read(timeout * 1000);
        request.timeout_write(timeout * 1000);
    }

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
        response.into_string().map_err(Error::from)
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
    use std::{env, time::Duration};
    use test_server::{self, helper, HttpResponse};

    #[test]
    #[serial]
    fn request_with_body_is_content_type_json() -> Result<(), Error> {
        let server = test_server::new("127.0.0.1:0", HttpResponse::BadRequest)?;
        env::set_var("K2_BASE_URL", server.url());
        init_config();

        let _ = request("", Some(json!({ "body": helper::random_string(100) })));
        let request = server.requests.next().unwrap();

        assert_eq!(
            "application/json",
            request.headers().get("content-type").unwrap()
        );

        env::remove_var("K2_BASE_URL");

        Ok(())
    }

    #[test]
    #[serial]
    fn send_request_body_if_given() -> Result<(), Error> {
        let server = test_server::new("127.0.0.1:0", HttpResponse::BadRequest)?;
        env::set_var("K2_BASE_URL", server.url());
        init_config();

        let body = json!({ "body": helper::random_string(100) });

        let _ = request("", Some(body.clone()));
        let request = server.requests.next().unwrap();

        assert_eq!(serde_json::to_vec(&body).unwrap(), &request.body()[..]);

        env::remove_var("K2_BASE_URL");

        Ok(())
    }

    #[test]
    #[serial]
    fn if_no_json_is_given_send_empty_request_body() -> Result<(), Error> {
        let server = test_server::new("127.0.0.1:0", HttpResponse::BadRequest)?;
        env::set_var("K2_BASE_URL", server.url());
        init_config();

        let _ = request("", None);
        let request = server.requests.next().unwrap();

        assert_eq!(b"", &request.body()[..]);

        env::remove_var("K2_BASE_URL");

        Ok(())
    }

    #[test]
    #[serial]
    fn http_is_using_timeout_from_config() -> Result<(), Error> {
        let server = test_server::new("127.0.0.1:0", || async {
            futures_timer::Delay::new(Duration::from_secs(5)).await;
            HttpResponse::Ok().body("foobar")
        })?;
        env::set_var("K2_BASE_URL", server.url());
        env::set_var("K2_TIMEOUT", "6");
        init_config();

        let res = request("", None)?;
        assert_eq!(res, "foobar");

        env::set_var("K2_TIMEOUT", "1");
        init_config();

        let res = request("", None).err();
        assert_eq!(
            format!("{}", res.unwrap()),
            "Request failed with status code 500"
        );

        env::remove_var("K2_BASE_URL");
        env::remove_var("K2_TIMEOUT");

        Ok(())
    }

    fn init_config() {
        let mut config_guard = CONFIG.write();
        *config_guard = Settings::init().unwrap();
        drop(config_guard);
    }
}
