use self::super::{MAP, StatusCode};
use self::super::super::http;

pub fn init(ctn: u16, pn: u16) -> StatusCode {
    // Do we know this CTN?
    if MAP.lock().unwrap().contains_key(&ctn) {
        error!("Card terminal has already been opened.");
        return StatusCode::ErrInvalid;
    }

    let path = format!("ct_init/{}/{}", ctn, pn);
    let response = http::request().post(&path, None).response();
    match response.status() {
        200 => handle_ok_status(response.body(), ctn, pn),
        _ => {
            error!("Request failed! Server response was not OK!");
            return StatusCode::ErrHtsi;
        }
    }
}

fn handle_ok_status(body: String, ctn: u16, pn: u16) -> StatusCode {
    match StatusCode::from_i8(body.parse::<i8>().unwrap()) {
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
        Err(why) => {
            error!("{}", why);
            StatusCode::ErrHtsi
        }
    }
}
