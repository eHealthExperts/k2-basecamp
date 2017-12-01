use self::super::{MAP, StatusCode};
use self::super::super::http;

pub fn close(ctn: u16) -> StatusCode {
    // Do we know this CTN?
    if !MAP.lock().unwrap().contains_key(&ctn) {
        error!("Card terminal has not been opened.");
        return StatusCode::ErrInvalid;
    }

    let pn = MAP.lock().unwrap().get(&ctn).unwrap().clone();
    let path = format!("ct_close/{}/{}", ctn, pn);
    let response = http::request(&path, None);
    match response {
        Err(why) => {
            error!("{}", why);
            StatusCode::ErrHtsi
        }
        Ok(res) => {
            match res.status {
                200 => handle_ok_status(res.body, ctn),
                _ => {
                    error!("Request failed! Server response was not OK!");
                    return StatusCode::ErrHtsi;
                }
            }
        }
    }
}

fn handle_ok_status(body: String, ctn: u16) -> StatusCode {
    match body.parse::<i8>() {
        Ok(status) => {
            match StatusCode::from_i8(status) {
                Ok(code) => {
                    match code {
                        StatusCode::Ok => {
                            // Remove CTN
                            MAP.lock().unwrap().remove(&ctn);
                            debug!("Card terminal has been closed.");
                            code
                        }
                        _ => code,
                    }
                }
                _ => {
                    error!("Unexpected server reponse body!");
                    StatusCode::ErrHtsi
                }
            }
        }
        _ => {
            error!("Unexpected server reponse body!");
            StatusCode::ErrHtsi
        }
    }
}

#[cfg(test)]
mod tests {

    use super::close;
    use super::super::MAP;
    use mockito;
    use mockito::mock;
    use rand;
    use std::env;

    #[test]
    fn call_close_when_server_not_up_returns_minus_128() {
        let ctn = rand::random::<u16>();
        let pn = rand::random::<u16>();

        MAP.lock().unwrap().insert(ctn, pn);

        env::set_var("K2_BASE_URL", mockito::SERVER_URL);

        assert_eq!(-128, close(ctn))
    }

    #[test]
    fn call_close_on_closed_terminal_returns_minus_1() {
        let ctn = rand::random::<u16>();
        assert_eq!(-1, close(ctn))
    }

    macro_rules! with_server_response {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (status, body, expected, map_contains_ctn) = $value;

                    let ctn = rand::random::<u16>();
                    let pn = rand::random::<u16>();

                    MAP.lock().unwrap().insert(ctn, pn);

                    let path = format!("/ct_close/{}/{}", ctn, pn);
                    let _mock = mock("POST", path.as_str())
                        .with_status(status)
                        .with_body(body)
                        .create();

                    env::set_var("K2_BASE_URL", mockito::SERVER_URL);

                    assert_eq!(expected, close(ctn));
                    assert_eq!(map_contains_ctn, MAP.lock().unwrap().contains_key(&ctn));

                    mockito::reset();
                }
            )*
        }
    }

    with_server_response! {
        not_found_returns_minus_128: (400, "Not found", -128, true),
        incorrect_response_returns_minus_128: (200, "hello world", -128, true),
        minus_1_response_returns_minus_1: (200, "-1", -1, true),
        zero_response_return_0: (200, "0", 0, false),
    }
}
