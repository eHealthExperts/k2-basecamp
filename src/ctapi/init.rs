use self::super::{ERR_HTSI, ERR_INVALID, MAP, OK};
use self::super::super::http;

pub fn init(ctn: u16, pn: u16) -> i8 {
    // Do we know this CTN?
    if MAP.lock().unwrap().contains_key(&ctn) {
        error!(
            "Card terminal has already been opened. Returning {}",
            ERR_INVALID
        );
        return ERR_INVALID;
    }

    let path = format!("ct_init/{}/{}", ctn, pn);
    let response = http::request().post(&path, None).response();
    if response.status() != 200 {
        error!("Request failed! Returning {}", ERR_HTSI);
        return ERR_HTSI;
    }

    handle_ok_status(response.body(), ctn, pn)
}

fn handle_ok_status(body: String, ctn: u16, pn: u16) -> i8 {
    let status = body.parse::<i8>().unwrap();
    if status == OK {
        // Store CTN
        MAP.lock().unwrap().insert(ctn, pn);
        debug!("Card terminal opened.");
    }

    debug!("Returning {}", status);
    status
}
