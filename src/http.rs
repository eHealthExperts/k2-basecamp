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
    use crate::{tests::random_string, Settings, CONFIG};
    use std::{env, time::Duration};
    use wiremock::{
        matchers::{body_json, body_string, header},
        Mock, MockServer, ResponseTemplate,
    };

    #[async_std::test]
    #[serial]
    async fn request_with_body_is_content_type_json() {
        let mock_server = MockServer::start().await;
        Mock::given(header("content-type", "application/json"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        env::set_var("K2_BASE_URL", mock_server.uri());

        init_config();

        let _ = request("", Some(json!({ "body": random_string(100) })));

        env::remove_var("K2_BASE_URL");
    }

    #[async_std::test]
    #[serial]
    async fn send_request_body_if_given() {
        let body = json!({ "body": random_string(100) });

        let mock_server = MockServer::start().await;
        Mock::given(body_json(&body))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        env::set_var("K2_BASE_URL", mock_server.uri());

        init_config();

        env::remove_var("K2_BASE_URL");
    }

    #[async_std::test]
    #[serial]
    async fn if_no_json_is_given_send_empty_request_body() {
        let mock_server = MockServer::start().await;
        Mock::given(body_string(""))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        env::set_var("K2_BASE_URL", mock_server.uri());

        let _ = request("", None);

        env::remove_var("K2_BASE_URL");
    }

    #[async_std::test]
    #[serial]
    async fn http_is_using_timeout_from_config() {
        let mock_server = MockServer::start().await;
        Mock::given(body_string("foobar"))
            .respond_with(ResponseTemplate::new(200).set_delay(Duration::from_secs(5)))
            .mount(&mock_server)
            .await;

        env::set_var("K2_BASE_URL", mock_server.uri());
        env::set_var("K2_TIMEOUT", "6");
        init_config();

        request("", None).ok();

        env::set_var("K2_TIMEOUT", "1");
        init_config();

        let res = request("", None).err();
        assert_eq!(
            format!("{}", res.unwrap()),
            "Request failed with status code 404"
        );

        env::remove_var("K2_BASE_URL");
        env::remove_var("K2_TIMEOUT");
    }

    fn init_config() {
        let mut config_guard = CONFIG.write();
        *config_guard = Settings::init().unwrap();
        drop(config_guard);
    }
}
