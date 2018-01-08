use self::super::MAP;
use self::super::super::{http, Status};

pub fn close(ctn: u16) -> Status {
    // Do we know this CTN?
    if !MAP.lock().contains_key(&ctn) {
        error!("Card terminal has not been opened.");
        return Status::ErrInvalid;
    }

    let pn = MAP.lock().get(&ctn).unwrap().clone();
    let path = format!("ct_close/{}/{}", ctn, pn);
    let response = http::request(&path, None);
    match response {
        Err(why) => {
            error!("{}", why);
            Status::ErrHtsi
        }
        Ok(res) => match res.status {
            200 => handle_ok_status(res.body, ctn),
            _ => {
                error!("Request failed! Server response was not OK!");
                return Status::ErrHtsi;
            }
        },
    }
}

fn handle_ok_status(body: String, ctn: u16) -> Status {
    let status: Status = match body.parse::<Status>() {
        Ok(status) => status,
        _ => {
            error!("Unexpected server reponse body!");
            return Status::ErrHtsi;
        }
    };

    match status {
        Status::OK => {
            // Remove CTN
            MAP.lock().remove(&ctn);
            debug!("Card terminal closed.");
            status
        }
        _ => status,
    }
}

#[cfg(test)]
mod tests {

    use super::close;
    use super::super::MAP;
    use rand;
    use std::env;
    use test_server::{self, http};

    #[test]
    fn returns_err_htsi_if_no_server() {
        env::set_var("K2_BASE_URL", "http://127.0.0.1:65432");

        let ctn = rand::random::<u16>();
        let pn = rand::random::<u16>();

        MAP.lock().insert(ctn, pn);

        assert_eq!(-128, close(ctn))
    }

    #[test]
    fn returns_err_invalid_if_already_closed() {
        let ctn = rand::random::<u16>();

        assert_eq!(-1, close(ctn))
    }

    #[test]
    fn use_ctn_and_pn_in_request_path() {
        let server = test_server::serve(None);
        server.reply().status(http::StatusCode::BAD_REQUEST);
        env::set_var("K2_BASE_URL", format!("http://{}", &server.addr()));

        let ctn = rand::random::<u16>();
        let pn = rand::random::<u16>();
        MAP.lock().insert(ctn, pn);

        close(ctn);

        let (parts, _body) = server.request().unwrap().into_parts();
        assert_eq!(parts.uri, *format!("/ct_close/{}/{}", ctn, pn));
    }

    #[test]
    fn returns_err_htsi_if_server_response_is_not_200() {
        let server = test_server::serve(None);
        server.reply().status(http::StatusCode::BAD_REQUEST);
        env::set_var("K2_BASE_URL", format!("http://{}", &server.addr()));

        let ctn = rand::random::<u16>();
        let pn = rand::random::<u16>();
        MAP.lock().insert(ctn, pn);

        assert_eq!(-128, close(ctn));
        assert_eq!(true, MAP.lock().contains_key(&ctn));
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
        MAP.lock().insert(ctn, pn);

        assert_eq!(-128, close(ctn));
        assert_eq!(true, MAP.lock().contains_key(&ctn));
    }

    #[test]
    fn returns_response_status_from_server() {
        let server = test_server::serve(None);
        server.reply().status(http::StatusCode::OK).body("-11");
        env::set_var("K2_BASE_URL", format!("http://{}", &server.addr()));

        let ctn = rand::random::<u16>();
        let pn = rand::random::<u16>();
        MAP.lock().insert(ctn, pn);

        assert_eq!(-11, close(ctn));
        assert_eq!(true, MAP.lock().contains_key(&ctn));
    }

    #[test]
    fn return_ok_and_close_ctn_if_server_returns_ok() {
        let server = test_server::serve(None);
        server.reply().status(http::StatusCode::OK).body("0");
        env::set_var("K2_BASE_URL", format!("http://{}", &server.addr()));

        let ctn = rand::random::<u16>();
        let pn = rand::random::<u16>();
        MAP.lock().insert(ctn, pn);

        assert_eq!(0, close(ctn));
        assert_eq!(false, MAP.lock().contains_key(&ctn));
    }
}
