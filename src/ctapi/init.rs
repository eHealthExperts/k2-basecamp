use self::super::{MAP, StatusCode};
use self::super::super::http;

pub fn init(ctn: u16, pn: u16) -> StatusCode {
    // Do we know this CTN?
    if MAP.lock().unwrap().contains_key(&ctn) {
        error!("Card terminal has already been opened.");
        return StatusCode::ErrInvalid;
    }

    let path = format!("ct_init/{}/{}", ctn, pn);
    let response = http::request(&path, None);
    match response {
        Err(why) => {
            error!("{}", why);
            StatusCode::ErrHtsi
        }
        Ok(res) => {
            match res.status {
                200 => handle_ok_status(res.body, ctn, pn),
                _ => {
                    error!("Request failed! Server response was not OK!");
                    return StatusCode::ErrHtsi;
                }
            }
        }
    }
}

fn handle_ok_status(body: String, ctn: u16, pn: u16) -> StatusCode {
    match body.parse::<i8>() {
        Ok(status) => {
            match StatusCode::from_i8(status) {
                Ok(code) => {
                    match code {
                        StatusCode::Ok => {
                            // Store CTN
                            MAP.lock().unwrap().insert(ctn, pn);
                            debug!("Card terminal opened.");
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

    use super::init;
    use super::super::MAP;
    use mockito;
    use mockito::mock;
    use rand;
    use std::env;

    #[test]
    fn call_init_when_server_not_up_returns_minus_128() {
        let ctn = rand::random::<u16>();
        let pn = rand::random::<u16>();

        env::set_var("K2_BASE_URL", mockito::SERVER_URL);

        assert_eq!(-128, init(ctn, pn))
    }

    #[test]
    fn call_init_on_open_terminal_returns_minus_1() {
        let ctn = rand::random::<u16>();
        let pn = rand::random::<u16>();

        MAP.lock().unwrap().insert(ctn, pn);

        assert_eq!(-1, init(ctn, pn))
    }

    macro_rules! with_server_response {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (status, body, expected, map_contains_ctn) = $value;

                    let ctn = rand::random::<u16>();
                    let pn = rand::random::<u16>();

                    let path = format!("/ct_init/{}/{}", ctn, pn);
                    let _mock = mock("POST", path.as_str())
                        .with_status(status)
                        .with_body(body)
                        .create();

                    env::set_var("K2_BASE_URL", mockito::SERVER_URL);

                    assert_eq!(expected, init(ctn, pn));
                    assert_eq!(map_contains_ctn, MAP.lock().unwrap().contains_key(&ctn));

                    mockito::reset();
                }
            )*
        }
    }

    with_server_response! {
        not_found_returns_minus_128: (400, "Not found", -128, false),
        incorrect_response_returns_minus_128: (200, "hello world", -128, false),
        minus_1_response_returns_minus_1: (200, "-1", -1, false),
        zero_response_return_0: (200, "0", 0, true),
    }
}
