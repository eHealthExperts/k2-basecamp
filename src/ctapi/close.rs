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
    let response = http::request().post(&path, None).response();
    match response.status() {
        200 => handle_ok_status(response.body(), ctn),
        _ => {
            error!("Request failed! Server response was not OK!");
            return StatusCode::ErrHtsi;
        }
    }
}

fn handle_ok_status(body: String, ctn: u16) -> StatusCode {
    match StatusCode::from_i8(body.parse::<i8>().unwrap()) {
        Some(code) => {
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
        None => {
            error!("Status code from server responses is not CTAPI conform!");
            StatusCode::ErrHtsi
        }
    }
}
