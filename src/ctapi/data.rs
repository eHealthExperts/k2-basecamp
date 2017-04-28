extern crate hyper;
extern crate serde_json;

pub use self::super::{ERR_HOST, ERR_HTSI, ERR_INVALID, MAP, OK};
pub use self::super::super::{http, logging};

use base64::{encode, decode};
use hyper::status::StatusCode;
use std::io::Read;
use std::slice;

#[derive(Serialize)]
struct RequestData {
    dad: u8,
    sad: u8,
    lenc: usize,
    command: String,
    lenr: usize,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct ResponseData {
    dad: u8,
    sad: u8,
    lenr: usize,
    response: String,
    responseCode: i8,
}

pub fn data(ctn: u16,
            dad: *mut u8,
            sad: *mut u8,
            lenc: usize,
            command: *const u8,
            lenr: *mut usize,
            response: *mut u8)
            -> i8 {
    logging::init();

    debug!("CT_data: Called");
    debug!(" ctn: {}", ctn);

    let _dad: &mut u8 = unsafe { &mut *dad };
    debug!(" dad: {}", _dad);

    let _sad: &mut u8 = unsafe { &mut *sad };
    debug!(" sad: {}", _sad);
    debug!(" lenc: {}", lenc);

    let _command = unsafe { slice::from_raw_parts(command, lenc as usize) };
    debug!(" command: {:?}", _command);

    let _lenr: &mut usize = unsafe { &mut *lenr };
    debug!(" lenr: {}", _lenr);

    let _response = unsafe { slice::from_raw_parts_mut(response, *_lenr) };
    debug!(" response.len(): {}", _response.len());

    if !MAP.lock().unwrap().contains_key(&ctn) {
        debug!("CT_data: Card terminal has not been opened. Returning {}",
               ERR_INVALID);
        return ERR_INVALID;
    }

    let request_data = RequestData {
        dad: *_dad,
        sad: *_sad,
        lenc: lenc,
        command: encode(_command),
        lenr: *_lenr,
    };

    let pn = MAP.lock().unwrap();
    let pn = pn.get(&ctn).unwrap();

    let mut path = String::from("ct_data");
    path.push_str("/");
    path.push_str(&ctn.to_string());
    path.push_str("/");
    path.push_str(&pn.to_string());

    let mut http_response = match http::post(path, &request_data) {
        Ok(http_response) => http_response,
        Err(error) => {
            debug!("Error: {:?}", error);
            error!("CT_data: Request failed! Returning {}", ERR_HTSI);
            return ERR_HTSI;
        }
    };

    debug!("{:?}", http_response); // TODO enrich output

    match http_response.status {
        StatusCode::Ok => {
            // decode server response
            let mut body = String::new();
            http_response.read_to_string(&mut body).unwrap();
            debug!("CT_data: Response body: {}", body);

            let response_data: ResponseData = serde_json::from_str(&body).unwrap();

            if response_data.responseCode == OK {
                *_dad = response_data.dad;
                *_sad = response_data.sad;
                *_lenr = response_data.lenr;

                let decoded = decode(&response_data.response).unwrap();
                debug!("CT_data: Decoded response field {:?}", decoded);

                for (place, element) in _response.iter_mut().zip(decoded.iter()) {
                    *place = *element;
                }
            }
            debug!("CT_data: Returning {}", response_data.responseCode);
            return response_data.responseCode;
        }
        _ => {
            error!("CT_data: Response not OK! Returning {}", ERR_HOST);
            ERR_HOST
        }
    }
}
