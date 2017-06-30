extern crate serde_json;

use self::super::{MAP, StatusCode};
use self::super::super::http;

use base64::{encode, decode};
use std::slice;

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct Response {
    dad: u8,
    sad: u8,
    lenr: u16,
    response: String,
    responseCode: i8,
}

pub fn data(
    ctn: u16,
    dad: *mut u8,
    sad: *mut u8,
    lenc: u16,
    command: *const u8,
    lenr: *mut u16,
    response: *mut u8,
) -> StatusCode {
    if !MAP.lock().unwrap().contains_key(&ctn) {
        error!("Card terminal has not been opened.");
        return StatusCode::ErrInvalid;
    }

    let safe_dad: &mut u8 = unsafe { &mut *dad };
    debug!("dad: {}", safe_dad);

    let safe_sad: &mut u8 = unsafe { &mut *sad };
    debug!("sad: {}", safe_sad);
    debug!("lenc: {}", lenc);

    let safe_command = unsafe { slice::from_raw_parts(command, lenc as usize) };
    debug!("command: {:?}", safe_command);

    let safe_lenr: &mut u16 = unsafe { &mut *lenr };
    debug!("lenr: {}", safe_lenr);

    let safe_response = unsafe { slice::from_raw_parts_mut(response, *safe_lenr as usize) };
    debug!("response with {} slices formed", safe_response.len());

    let json = format!(
        "{{\"dad\":{},\"sad\":{},\"lenc\":{},\"command\":\"{}\",\"lenr\":{}}}",
        *safe_dad,
        *safe_sad,
        lenc,
        encode(safe_command),
        *safe_lenr
    );

    let pn = MAP.lock().unwrap().get(&ctn).unwrap().clone();
    let path = format!("ct_data/{}/{}", ctn, pn);
    let response = http::request().post(&path, Some(json)).response();
    match response.status() {
        200 => {
            match serde_json::from_str(&response.body()) {
                Ok(mut data) => {
                    data = data as Response;
                    match StatusCode::from_i8(data.responseCode) {
                        Ok(code) => {
                            match code {
                                StatusCode::Ok => {
                                    *safe_dad = data.dad;
                                    *safe_sad = data.sad;
                                    *safe_lenr = data.lenr;

                                    let decoded = decode(&data.response).unwrap();
                                    debug!("Decoded response field {:?}", decoded);

                                    for (place, element) in safe_response.iter_mut().zip(
                                        decoded.iter(),
                                    )
                                    {
                                        *place = *element;
                                    }
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
                Err(why) => {
                    error!("Failed to parse server response data. {:?}", why);
                    StatusCode::ErrHtsi
                }
            }
        }
        _ => {
            error!("Request failed! Server response was not OK!");
            return StatusCode::ErrHtsi;
        }
    }
}
