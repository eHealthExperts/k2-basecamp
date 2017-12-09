use self::super::{MAP, Status};
use self::super::super::http;

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
            error!("{}", why);
            Status::ErrHtsi
        }
        Ok(res) => {
            match res.status {
                200 => handle_ok_status(res.body, ctn, pn),
                _ => {
                    error!("Request failed! Server response was not OK!");
                    return Status::ErrHtsi;
                }
            }
        }
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
            debug!("Card terminal opened.");
            status
        }
        _ => status,
    }
}

#[cfg(test)]
mod tests {

    use super::init;
    use super::super::MAP;
    use antidote::Mutex;
    use rand;
    use rouille::Response;

    #[test]
    fn returns_err_htsi_if_no_server() {
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
    fn returns_err_htsi_if_server_response_is_not_200() {
        let ctn = rand::random::<u16>();
        let pn = rand::random::<u16>();

        let param = Mutex::new(hashmap!["ctn" => ctn, "pn" => pn]);

        let shutdown = test_server!({
            (POST) (/ct_init/{ctn: u16}/{pn: u16}) => {
                assert_eq!(&ctn, param.lock().get("ctn").unwrap());
                assert_eq!(&pn, param.lock().get("pn").unwrap());

                Response::empty_404()
            }
        });

        assert_eq!(-128, init(ctn, pn));
        assert_eq!(false, MAP.lock().contains_key(&ctn));

        // kill server thread
        let _ = shutdown.send(());
    }

    #[test]
    fn returns_err_htsi_if_server_response_not_contains_status() {
        let ctn = rand::random::<u16>();
        let pn = rand::random::<u16>();

        let param = Mutex::new(hashmap!["ctn" => ctn, "pn" => pn]);

        let shutdown = test_server!({
            (POST) (/ct_init/{ctn: u16}/{pn: u16}) => {
                assert_eq!(&ctn, param.lock().get("ctn").unwrap());
                assert_eq!(&pn, param.lock().get("pn").unwrap());

                Response::text("hello world")
            }
        });

        assert_eq!(-128, init(ctn, pn));
        assert_eq!(false, MAP.lock().contains_key(&ctn));

        // kill server thread
        let _ = shutdown.send(());
    }

    #[test]
    fn returns_response_status_from_server() {
        let ctn = rand::random::<u16>();
        let pn = rand::random::<u16>();

        let param = Mutex::new(hashmap!["ctn" => ctn, "pn" => pn]);

        let shutdown = test_server!({
            (POST) (/ct_init/{ctn: u16}/{pn: u16}) => {
                assert_eq!(&ctn, param.lock().get("ctn").unwrap());
                assert_eq!(&pn, param.lock().get("pn").unwrap());

                Response::text("-11")
            }
        });

        assert_eq!(-11, init(ctn, pn));
        assert_eq!(false, MAP.lock().contains_key(&ctn));

        // kill server thread
        let _ = shutdown.send(());
    }

    #[test]
    fn returns_ok_and_init_ctn_if_server_returns_ok() {
        let ctn = rand::random::<u16>();
        let pn = rand::random::<u16>();

        let param = Mutex::new(hashmap!["ctn" => ctn, "pn" => pn]);

        let shutdown = test_server!({
            (POST) (/ct_init/{ctn: u16}/{pn: u16}) => {
                assert_eq!(&ctn, param.lock().get("ctn").unwrap());
                assert_eq!(&pn, param.lock().get("pn").unwrap());

                Response::text("0")
            }
        });

        assert_eq!(0, init(ctn, pn));
        assert_eq!(true, MAP.lock().contains_key(&ctn));

        // kill server thread
        let _ = shutdown.send(());
    }
}
