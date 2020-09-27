use crate::ctapi::MAP;
use crate::{http, Status, CONFIG};

pub fn init(mut ctn: u16, mut pn: u16) -> anyhow::Result<Status> {
    if let (Some(ctn_from_cfg), Some(pn_from_cfg)) = (CONFIG.read().ctn, CONFIG.read().pn) {
        debug!(
            "Use ctn '{}' and pn '{}' from configuration.",
            ctn_from_cfg, pn_from_cfg
        );
        ctn = ctn_from_cfg;
        pn = pn_from_cfg;
    }

    // Do we know this CTN?
    if MAP.read().contains_key(&ctn) {
        error!("Card terminal has already been opened.");
        return Ok(Status::ERR_INVALID);
    }

    let path = format!("ct_init/{}/{}", ctn, pn);
    let response = http::request(&path, None)?;

    match response.parse::<i8>() {
        Ok(status_code) => {
            let status = Status::from(status_code);
            if let Status::OK = status {
                // Store CTN
                let _ = MAP.write().insert(ctn, pn);
                info!("Card terminal opened.");
            }

            Ok(status)
        }
        Err(why) => {
            debug!("{}", why);
            Err(format_err!("Unexpected server response found in body!"))
        }
    }
}

#[cfg(test)]
mod tests {

    use super::init;
    use crate::{ctapi::MAP, Status};
    use std::env::{remove_var, set_var};
    use wiremock::{matchers, Mock, MockServer, ResponseTemplate};

    #[test]
    #[serial]
    fn returns_err_if_no_server() {
        set_var("K2_BASE_URL", "http://127.0.0.1:65432");
        crate::tests::init_config_clear_map();

        let ctn = rand::random::<u16>();
        let pn = rand::random::<u16>();

        assert!(init(ctn, pn).is_err());

        remove_var("K2_BASE_URL");
    }

    #[test]
    #[serial]
    fn returns_err_invalid_if_already_open() {
        crate::tests::init_config_clear_map();

        let ctn = rand::random::<u16>();
        let pn = rand::random::<u16>();

        let _ = MAP.write().insert(ctn, pn);

        assert_eq!(Some(Status::ERR_INVALID), init(ctn, pn).ok());
    }

    #[async_std::test]
    #[serial]
    async fn use_ctn_and_pn_in_request_path() {
        let ctn = rand::random::<u16>();
        let pn = rand::random::<u16>();

        let mock_server = MockServer::start().await;
        Mock::given(matchers::path(format!("/ct_init/{}/{}", ctn, pn)))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;
        set_var("K2_BASE_URL", mock_server.uri());
        crate::tests::init_config_clear_map();

        let _ = init(ctn, pn);

        remove_var("K2_BASE_URL");
    }
    #[async_std::test]
    #[serial]
    async fn use_ctn_and_pn_from_config() {
        let ctn = rand::random::<u16>();
        set_var("K2_CTN", format!("{}", ctn));
        let pn = rand::random::<u16>();
        set_var("K2_PN", format!("{}", pn));
        crate::tests::init_config_clear_map();

        let mock_server = MockServer::start().await;
        Mock::given(matchers::path(format!("/ct_init/{}/{}", ctn, pn)))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;
        set_var("K2_BASE_URL", mock_server.uri());

        let unused_ctn = rand::random::<u16>();
        let unused_pn = rand::random::<u16>();

        let _ = init(unused_ctn, unused_pn);

        remove_var("K2_BASE_URL");
        remove_var("K2_CTN");
        remove_var("K2_PN");
    }

    #[async_std::test]
    #[serial]
    async fn returns_err_if_server_response_is_not_200() {
        let mock_server = MockServer::start().await;
        Mock::given(matchers::any())
            .respond_with(ResponseTemplate::new(400))
            .mount(&mock_server)
            .await;
        set_var("K2_BASE_URL", mock_server.uri());

        crate::tests::init_config_clear_map();

        let ctn = rand::random::<u16>();
        let pn = rand::random::<u16>();

        assert!(init(ctn, pn).is_err());
        assert!(!MAP.read().contains_key(&ctn));

        remove_var("K2_BASE_URL");
    }

    #[async_std::test]
    #[serial]
    async fn returns_err_if_server_response_not_contains_status() {
        let mock_server = MockServer::start().await;
        Mock::given(matchers::any())
            .respond_with(ResponseTemplate::new(200).set_body_string("hello world"))
            .mount(&mock_server)
            .await;
        set_var("K2_BASE_URL", mock_server.uri());

        crate::tests::init_config_clear_map();

        let ctn = rand::random::<u16>();
        let pn = rand::random::<u16>();

        assert!(init(ctn, pn).is_err());
        assert!(!MAP.read().contains_key(&ctn));

        remove_var("K2_BASE_URL");
    }

    #[async_std::test]
    #[serial]
    async fn returns_response_status_from_server() {
        let mock_server = MockServer::start().await;
        Mock::given(matchers::any())
            .respond_with(ResponseTemplate::new(200).set_body_json(-11))
            .mount(&mock_server)
            .await;
        set_var("K2_BASE_URL", mock_server.uri());

        crate::tests::init_config_clear_map();

        let ctn = rand::random::<u16>();
        let pn = rand::random::<u16>();

        assert_eq!(Some(Status::ERR_MEMORY), init(ctn, pn).ok());

        remove_var("K2_BASE_URL");
    }

    #[async_std::test]
    #[serial]
    async fn returns_ok_and_init_ctn_if_server_returns_ok() {
        let mock_server = MockServer::start().await;
        Mock::given(matchers::any())
            .respond_with(ResponseTemplate::new(200).set_body_json(0))
            .mount(&mock_server)
            .await;
        set_var("K2_BASE_URL", mock_server.uri());

        crate::tests::init_config_clear_map();

        let ctn = rand::random::<u16>();
        let pn = rand::random::<u16>();

        assert_eq!(Some(Status::OK), init(ctn, pn).ok());
        assert!(MAP.read().contains_key(&ctn));

        remove_var("K2_BASE_URL");
    }
}
