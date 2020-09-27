use crate::CONFIG;
use serde_json::Value;
use std::time::Duration;

pub fn request(path: &str, request_body: Option<Value>) -> anyhow::Result<String> {
    let builder = ureq::builder();
    let agent = match CONFIG.read().timeout {
        None => builder.build(),
        Some(timeout) => builder.timeout(Duration::from_secs(timeout)).build(),
    };

    let url = format!("{}{}", CONFIG.read().base_url, path);
    debug!("Request URL: {}", url);
    let request = agent.post(&url).set("Content-Type", "application/json");

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

    match response {
        Ok(res) => res.into_string().map_err(anyhow::Error::from),
        Err(ureq::Error::Status(code, response)) => {
            debug!("{:?}", response);
            Err(format_err!("Request failed with status code {}", code))
        }
        Err(why) => {
            debug!("{:?}", why);
            Err(format_err!("Request failed with unknown error",))
        }
    }
}

#[cfg(test)]
mod tests {

    use super::request;
    use crate::{Settings, CONFIG};
    use std::{env, time::Duration};
    use test_server::{self, helper, HttpResponse};

    #[test]
    #[serial]
    fn request_with_body_is_content_type_json() -> anyhow::Result<()> {
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
    fn send_request_body_if_given() -> anyhow::Result<()> {
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
    fn if_no_json_is_given_send_empty_request_body() -> anyhow::Result<()> {
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
    fn http_is_using_timeout_from_config() -> anyhow::Result<()> {
        let server = test_server::new("127.0.0.1:0", || {
            std::thread::sleep(Duration::from_secs(5));
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
            "Request failed with unknown error"
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
