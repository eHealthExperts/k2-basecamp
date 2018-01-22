use self::super::MAP;
use self::super::super::{http, Status};

pub fn init(ctn: u16, pn: u16) -> Status {
    // Do we know this CTN?
    if MAP.lock().contains_key(&ctn) {
        error!("Card terminal has already been opened.");
        return Status::ErrInvalid;
    }

    let path = format!("ct_init/{}/{}", ctn, pn);
    let response = http::request(&path, None);
    match response {
        Err(why) => {
            error!("Request failed!");
            debug!("{}", why);
            Status::ErrHtsi
        }
        Ok(res) => match res.status {
            200 => handle_ok_status(res.body, ctn, pn),
            _ => {
                error!("Request failed! Server response was not OK!");
                return Status::ErrHtsi;
            }
        },
    }
}

fn handle_ok_status(body: String, ctn: u16, pn: u16) -> Status {
    let status: Status = match body.parse::<Status>() {
        Ok(status) => status,
        _ => {
            error!("Unexpected server reponse body!");
            Status::ErrHtsi
        }
    };

    match status {
        Status::OK => {
            // Store CTN
            MAP.lock().insert(ctn, pn);
            info!("Card terminal opened.");
            status
        }
        _ => status,
    }
}

#[cfg(test)]
mod tests {

    use super::init;
    use super::super::MAP;
    use rand;
    use std::env;
    use test_server::{self, http};

    #[test]
    fn returns_err_htsi_if_no_server() {
        env::set_var("K2_BASE_URL", "http://127.0.0.1:65432");

        let ctn = rand::random::<u16>();
        let pn = rand::random::<u16>();

        assert_eq!(-128, init(ctn, pn))
    }

    #[test]
    fn returns_err_invalid_if_already_open() {
        let ctn = rand::random::<u16>();
        let pn = rand::random::<u16>();

        MAP.lock().insert(ctn, pn);

        assert_eq!(-1, init(ctn, pn))
    }

    #[test]
    fn use_ctn_and_pn_in_request_path() {
        let server = test_server::serve(None);
        server.reply().status(http::StatusCode::BAD_REQUEST);
        env::set_var("K2_BASE_URL", format!("http://{}", &server.addr()));

        let ctn = rand::random::<u16>();
        let pn = rand::random::<u16>();

        init(ctn, pn);

        let (parts, _body) = server.request().unwrap().into_parts();
        assert_eq!(parts.uri, *format!("/ct_init/{}/{}", ctn, pn));
    }

    #[test]
    fn returns_err_htsi_if_server_response_is_not_200() {
        let server = test_server::serve(None);
        server.reply().status(http::StatusCode::BAD_REQUEST);
        env::set_var("K2_BASE_URL", format!("http://{}", &server.addr()));

        let ctn = rand::random::<u16>();
        let pn = rand::random::<u16>();

        assert_eq!(-128, init(ctn, pn));
        assert_eq!(false, MAP.lock().contains_key(&ctn));
    }

    #[test]
    fn returns_err_htsi_if_server_response_not_contains_status() {
        let server = test_server::serve(None);
        server
            .reply()
            .status(http::StatusCode::OK)
            .body("hello world");
        env::set_var("K2_BASE_URL", format!("http://{}", &server.addr()));

        let ctn = rand::random::<u16>();
        let pn = rand::random::<u16>();

        assert_eq!(-128, init(ctn, pn));
        assert_eq!(false, MAP.lock().contains_key(&ctn));
    }

    #[test]
    fn returns_response_status_from_server() {
        let server = test_server::serve(None);
        server.reply().status(http::StatusCode::OK).body("-11");
        env::set_var("K2_BASE_URL", format!("http://{}", &server.addr()));

        let ctn = rand::random::<u16>();
        let pn = rand::random::<u16>();

        assert_eq!(-11, init(ctn, pn));
    }

    #[test]
    fn returns_ok_and_init_ctn_if_server_returns_ok() {
        let server = test_server::serve(None);
        server.reply().status(http::StatusCode::OK).body("0");
        env::set_var("K2_BASE_URL", format!("http://{}", &server.addr()));

        let ctn = rand::random::<u16>();
        let pn = rand::random::<u16>();

        assert_eq!(0, init(ctn, pn));
        assert_eq!(true, MAP.lock().contains_key(&ctn));
    }
}
