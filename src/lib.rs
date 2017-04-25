extern crate base64;
extern crate hyper;
extern crate libc;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use base64::{encode, decode};
use hyper::Client;
use hyper::Error;
use hyper::client::response::Response;
use hyper::header::{Headers, ContentType};
use hyper::mime::{Mime, TopLevel, SubLevel};
use hyper::status::StatusCode;
use libc::{uint8_t, size_t};
use log::LogLevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;
use serde::Serialize;
use std::collections::HashMap;
use std::env::var;
use std::io::Read;
use std::slice;
use std::sync::{Once, ONCE_INIT, Mutex};

const BASE_URL: &'static str = "http://localhost:8080/k2/ctapi/";

static OK: i8 = 0;
static ERR_INVALID: i8 = -1;
static ERR_HOST: i8 = -127;
static ERR_HTSI: i8 = -128;

static INIT: Once = ONCE_INIT;

lazy_static! {
    static ref MAP: Mutex<HashMap<u16, u16>> = Mutex::new(HashMap::new());
}

#[derive(Serialize)]
struct Empty();

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

macro_rules! post_request {
    ($path:expr) => (post_request($path, &Empty{}));
    ($path:expr, $data:expr) => (post_request($path, $data));
}

#[no_mangle]
#[allow(non_snake_case, unused_must_use)]
pub extern "C" fn CT_init(ctn: u16, pn: u16) -> i8 {
    init_logging();

    debug!("CT_init: Called (ctn {}, pn {})", ctn, pn);

    // Do we know this CTN?
    if MAP.lock().unwrap().contains_key(&ctn) {
        debug!("CT_init: Card terminal has already been opened. Returning {}",
               ERR_INVALID);
        return ERR_INVALID;
    }

    // Build the request URL
    let endpoint = "ct_init".to_string();
    let path = endpoint + "/" + &ctn.to_string() + "/" + &pn.to_string();

    // Perform the request
    match post_request!(&path) {
        Ok(mut response) => {
            debug!("{:?}", response); // TODO enrich output

            match response.status {
                StatusCode::Ok => {
                    // Cast server response
                    let mut body = String::new();
                    response.read_to_string(&mut body).unwrap();

                    let status = body.parse::<i8>().unwrap();
                    if status == OK {
                        // Store CTN
                        MAP.lock().unwrap().insert(ctn, pn);
                        debug!("CT_init: Card terminal opened.");
                    }

                    debug!("CT_init: Returning {}", status);
                    status
                }
                _ => {
                    error!("CT_init: Response not OK! Returning {}", ERR_HOST);
                    ERR_HOST
                }
            }
        }
        Err(error) => {
            debug!("Error: {:?}", error);
            error!("CT_data: Request failed! Returning {}", ERR_HTSI);
            ERR_HTSI
        }
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn CT_data(ctn: u16,
                          dad: *mut uint8_t,
                          sad: *mut uint8_t,
                          lenc: size_t,
                          command: *const uint8_t,
                          lenr: *mut size_t,
                          response: *mut uint8_t)
                          -> i8 {
    init_logging();

    debug!("CT_data: Called");
    debug!(" ctn: {}", ctn);

    let _dad: &mut u8 = unsafe { &mut *dad };
    debug!(" dad: {}", _dad);

    let _sad: &mut u8 = unsafe { &mut *sad };
    debug!(" sad: {}", _sad);
    debug!(" lenc: {}", lenc);

    let _command = unsafe { slice::from_raw_parts(command, lenc as usize) };
    debug!(" command: {:?}", _command);

    let _lenr: &mut size_t = unsafe { &mut *lenr };
    debug!(" lenr: {}", _lenr);

    let _response = unsafe { slice::from_raw_parts_mut(response, *_lenr) };
    debug!(" response.len(): {}", _response.len());

    if !MAP.lock().unwrap().contains_key(&ctn) {
        debug!("CT_data: Card terminal has not been opened. Returning {}",
               ERR_INVALID);
        return ERR_INVALID;
    }

    let requestData = RequestData {
        dad: *_dad,
        sad: *_sad,
        lenc: lenc,
        command: encode(_command),
        lenr: *_lenr,
    };

    let pn = MAP.lock().unwrap();
    let pn = pn.get(&ctn).unwrap();
    let endpoint = "ct_data".to_string();
    let path = endpoint + "/" + &ctn.to_string() + "/" + &pn.to_string();

    match post_request(&path, &requestData) {
        Ok(mut http_response) => {
            debug!("{:?}", http_response); // TODO enrich output

            match http_response.status {
                StatusCode::Ok => {
                    // decode server response
                    let mut body = String::new();
                    http_response.read_to_string(&mut body).unwrap();
                    debug!("CT_data: Response body: {}", body);

                    let responseData: ResponseData = serde_json::from_str(&body).unwrap();

                    if responseData.responseCode == OK {
                        *_dad = responseData.dad;
                        *_sad = responseData.sad;
                        *_lenr = responseData.lenr;

                        let decoded = decode(&responseData.response).unwrap();
                        debug!("CT_data: Decoded response field {:?}", decoded);

                        for (place, element) in _response.iter_mut().zip(decoded.iter()) {
                            *place = *element;
                        }
                    }
                    debug!("CT_data: Returning {}", responseData.responseCode);
                    return responseData.responseCode;
                }
                _ => {
                    error!("CT_data: Response not OK! Returning {}", ERR_HOST);
                    ERR_HOST
                }
            }
        }
        Err(error) => {
            debug!("Error: {:?}", error);
            error!("CT_data: Request failed! Returning {}", ERR_HTSI);
            ERR_HTSI
        }
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn CT_close(ctn: u16) -> i8 {
    init_logging();

    debug!("CT_close: Called (ctn {})", ctn);

    // Do we know this CTN?
    if !MAP.lock().unwrap().contains_key(&ctn) {
        debug!("CT_close: Card terminal has not been opened. Returning {}",
               ERR_INVALID);
        return ERR_INVALID;
    }

    // Build the request URL
    let pn = MAP.lock().unwrap().get(&ctn).unwrap().clone();
    let endpoint = "ct_close".to_string();
    let path = endpoint + "/" + &ctn.to_string() + "/" + &pn.to_string();

    // Perform the request
    let mut response = post_request!(&path);

    match response.status {
        StatusCode::Ok => {
            // Cast server response
            let mut body = String::new();
            response.read_to_string(&mut body).unwrap();

            let status = body.parse::<i8>().unwrap();
            if status == OK {
                // Remove CTN
                MAP.lock().unwrap().remove(&ctn);
                debug!("CT_close: Card terminal has been closed.");
            }

            debug!("CT_close: Returning {}", status);
            status
        }
        _ => {
            error!("CT_close: Response not OK! Returning {}", ERR_HOST);
            ERR_HOST
        }
    }
}

fn init_logging() {
    match var("K2_LOG_PATH") {
        Ok(path) => {
            INIT.call_once(|| {
                let file = FileAppender::builder()
                    .encoder(Box::new(PatternEncoder::new("{d} {l} {M}: {m}{n}")))
                    .build(path + &"/ctehxk2.log".to_string())
                    .unwrap();

                let config = Config::builder()
                    .appender(Appender::builder().build("file", Box::new(file)))
                    .logger(Logger::builder()
                                .appender("file")
                                .additive(false)
                                .build("ctehxk2", LogLevelFilter::Debug))
                    .build(Root::builder()
                               .appender("file")
                               .build(LogLevelFilter::Error))
                    .unwrap();

                log4rs::init_config(config).unwrap();
            })
        }
        _ => (),
    }
}

fn post_request<T>(path: &str, payload: &T) -> Result<Response, Error>
    where T: Serialize
{
    let base_url = var("K2_BASE_URL").unwrap_or(BASE_URL.to_string());
    let url = base_url + path;

    let client = Client::new();

    let mut headers = Headers::new();
    headers.set(ContentType(Mime(TopLevel::Application, SubLevel::Json, vec![])));

    let body = serde_json::to_string(&payload).unwrap();

    debug!("HTTP POST URL: {}", url);
    debug!("HTTP POST body: {:?}", body);

    let mut builder = client.post(&url);
    builder = builder.headers(headers);
    builder = builder.body(&body[..]);
    builder.send()
}
