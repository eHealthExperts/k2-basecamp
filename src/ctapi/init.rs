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
    use std::env;
    use test_server::{self, HttpResponse};

    #[test]
    #[serial]
    fn returns_err_if_no_server() {
        env::set_var("K2_BASE_URL", "http://127.0.0.1:65432");
        crate::tests::init_config_clear_map();

        let ctn = rand::random::<u16>();
        let pn = rand::random::<u16>();

        assert!(init(ctn, pn).is_err());

        env::remove_var("K2_BASE_URL");
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

    #[test]
    #[serial]
    fn use_ctn_and_pn_in_request_path() -> anyhow::Result<()> {
        let server = test_server::new("127.0.0.1:0", HttpResponse::BadRequest)?;
        env::set_var("K2_BASE_URL", server.url());
        crate::tests::init_config_clear_map();

        let ctn = rand::random::<u16>();
        let pn = rand::random::<u16>();

        let _ = init(ctn, pn);

        let req = server.requests.next().unwrap();
        assert_eq!(req.uri().path(), &format!("/ct_init/{}/{}", ctn, pn));

        env::remove_var("K2_BASE_URL");

        Ok(())
    }

    #[test]
    #[serial]
    fn use_ctn_and_pn_from_config() -> anyhow::Result<()> {
        let server = test_server::new("127.0.0.1:0", HttpResponse::BadRequest)?;
        env::set_var("K2_BASE_URL", server.url());
        let ctn = rand::random::<u16>();
        env::set_var("K2_CTN", format!("{}", ctn));
        let pn = rand::random::<u16>();
        env::set_var("K2_PN", format!("{}", pn));
        crate::tests::init_config_clear_map();

        let unused_ctn = rand::random::<u16>();
        let unused_pn = rand::random::<u16>();

        let _ = init(unused_ctn, unused_pn);

        let req = server.requests.next().unwrap();
        assert_eq!(req.uri().path(), &format!("/ct_init/{}/{}", ctn, pn));

        env::remove_var("K2_BASE_URL");
        env::remove_var("K2_CTN");
        env::remove_var("K2_PN");

        Ok(())
    }

    #[test]
    #[serial]
    fn returns_err_if_server_response_is_not_200() -> anyhow::Result<()> {
        let server = test_server::new("127.0.0.1:0", HttpResponse::BadRequest)?;
        env::set_var("K2_BASE_URL", server.url());
        crate::tests::init_config_clear_map();

        let ctn = rand::random::<u16>();
        let pn = rand::random::<u16>();

        assert!(init(ctn, pn).is_err());
        assert_eq!(false, MAP.read().contains_key(&ctn));

        env::remove_var("K2_BASE_URL");

        Ok(())
    }

    #[test]
    #[serial]
    fn returns_err_if_server_response_not_contains_status() -> anyhow::Result<()> {
        let server = test_server::new("127.0.0.1:0", || HttpResponse::Ok().body("hello world"))?;
        env::set_var("K2_BASE_URL", server.url());
        crate::tests::init_config_clear_map();

        let ctn = rand::random::<u16>();
        let pn = rand::random::<u16>();

        assert!(init(ctn, pn).is_err());
        assert_eq!(false, MAP.read().contains_key(&ctn));

        env::remove_var("K2_BASE_URL");

        Ok(())
    }

    #[test]
    #[serial]
    fn returns_response_status_from_server() -> anyhow::Result<()> {
        let server = test_server::new("127.0.0.1:0", || HttpResponse::Ok().body("-11"))?;
        env::set_var("K2_BASE_URL", server.url());
        crate::tests::init_config_clear_map();

        let ctn = rand::random::<u16>();
        let pn = rand::random::<u16>();

        assert_eq!(Some(Status::ERR_MEMORY), init(ctn, pn).ok());

        env::remove_var("K2_BASE_URL");

        Ok(())
    }

    #[test]
    #[serial]
    fn returns_ok_and_init_ctn_if_server_returns_ok() -> anyhow::Result<()> {
        let server = test_server::new("127.0.0.1:0", || HttpResponse::Ok().body("0"))?;
        env::set_var("K2_BASE_URL", server.url());
        crate::tests::init_config_clear_map();

        let ctn = rand::random::<u16>();
        let pn = rand::random::<u16>();

        assert_eq!(Some(Status::OK), init(ctn, pn).ok());
        assert_eq!(true, MAP.read().contains_key(&ctn));

        env::remove_var("K2_BASE_URL");

        Ok(())
    }
}
