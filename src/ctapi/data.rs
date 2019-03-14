use crate::ctapi::MAP;
use crate::{http, Status, CONFIG};
use data_encoding::{BASE64, HEXLOWER};
use serde_json;
use std::slice;

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct Response {
    dad: u8,
    sad: u8,
    lenr: u16,
    response: String,
    #[serde(rename = "responseCode")]
    status: i8,
}

pub fn data(
    mut ctn: u16,
    dad: *mut u8,
    sad: *mut u8,
    lenc: u16,
    command: *const u8,
    lenr: *mut u16,
    response: *mut u8,
) -> Status {
    if let Some(ctn_from_cfg) = CONFIG.ctn {
        debug!("Use ctn '{}' from configuration", ctn_from_cfg);
        ctn = ctn_from_cfg;
    }

    if !MAP.lock().contains_key(&ctn) {
        error!("Card terminal has not been opened.");
        return Status::ErrInvalid;
    }

    let pn = match MAP.lock().get(&ctn) {
        Some(pn) => *pn,
        None => {
            error!("Failed to extract pn for given ctn!");
            return Status::ErrHtsi;
        }
    };

    let safe_dad: &mut u8 = unsafe { &mut *dad };
    debug!("dad: {}", safe_dad);

    let safe_sad: &mut u8 = unsafe { &mut *sad };
    debug!("sad: {}", safe_sad);
    debug!("lenc: {}", lenc);

    let safe_command = unsafe { slice::from_raw_parts(command, lenc as usize) };
    debug!("command: {:?}", HEXLOWER.encode(safe_command));

    let safe_lenr: &mut u16 = unsafe { &mut *lenr };
    debug!("lenr: {}", safe_lenr);

    let safe_response = unsafe { slice::from_raw_parts_mut(response, *safe_lenr as usize) };
    debug!("response with {} slices formed", safe_response.len());

    let json = json!({
        "dad": *safe_dad,
        "sad": *safe_sad,
        "lenc": lenc,
        "command": BASE64.encode(safe_command),
        "lenr": *safe_lenr
    });

    let path = format!("ct_data/{}/{}", ctn, pn);
    let response = http::request(&path, Some(json));
    let res = match response {
        Ok(response) => response,
        Err(why) => {
            error!("Request failed!");
            debug!("{}", why);
            return Status::ErrHtsi;
        }
    };

    if res.status != 200 {
        error!("Request failed! Server response was not OK!");
        return Status::ErrHtsi;
    }

    let json: Response = match serde_json::from_str(&res.body) {
        Ok(json) => json,
        Err(why) => {
            error!("Failed to parse server response data!");
            debug!("{}", why);
            return Status::ErrHtsi;
        }
    };

    let status: Status = json.status.into();
    match status {
        Status::OK => {
            *safe_dad = json.dad;
            *safe_sad = json.sad;
            *safe_lenr = json.lenr;

            let decoded = match BASE64.decode(&json.response.into_bytes()) {
                Ok(content) => {
                    debug!("Decoded response field: {:?}", HEXLOWER.encode(&content));
                    content
                }
                Err(why) => {
                    error!("Failed to extract response.");
                    debug!("{}", why);
                    return Status::ErrHtsi;
                }
            };

            for (place, element) in safe_response.iter_mut().zip(decoded.iter()) {
                *place = *element;
            }

            status
        }
        _ => status,
    }
}

#[cfg(test)]
mod tests {

    use super::super::MAP;
    use super::data;
    use data_encoding::BASE64;
    use rand;
    use serde_json::{self, Value};
    use std::env;
    use std::slice;
    use std::u16::MAX;
    use test_server::{self, HttpResponse};

    #[test]
    fn returns_err_invalid_if_terminal_closed() {
        let (_, command, lenc, response, mut lenr, mut dad, mut sad, ctn, _) = rand_params();

        assert_eq!(
            -1,
            data(ctn, &mut dad, &mut sad, lenc, command, &mut lenr, response,)
        )
    }

    #[test]
    fn returns_err_htsi_if_no_server() {
        env::set_var("K2_BASE_URL", "http://127.0.0.1:65432");

        let (_, command, lenc, response, mut lenr, mut dad, mut sad, ctn, pn) = rand_params();

        MAP.lock().insert(ctn, pn);

        assert_eq!(
            -128,
            data(ctn, &mut dad, &mut sad, lenc, command, &mut lenr, response)
        )
    }

    #[test]
    fn use_ctn_and_pn_in_request_path() {
        let server = test_server::new(0, |_| HttpResponse::BadRequest().into());
        env::set_var("K2_BASE_URL", server.url());

        let (_, command, lenc, response, mut lenr, mut dad, mut sad, ctn, pn) = rand_params();
        MAP.lock().insert(ctn, pn);

        data(ctn, &mut dad, &mut sad, lenc, command, &mut lenr, response);

        let path = server.requests.next().unwrap().path;
        assert_eq!(path, *format!("/ct_data/{}/{}", ctn, pn));
    }

    #[test]
    fn post_body_contains_parameter() {
        let server = test_server::new(0, |_| HttpResponse::BadRequest().into());
        env::set_var("K2_BASE_URL", server.url());

        let (command, command_ptr, lenc, response, mut lenr, mut dad, mut sad, ctn, pn) =
            rand_params();

        MAP.lock().insert(ctn, pn);

        assert_eq!(
            -128,
            data(
                ctn,
                &mut dad,
                &mut sad,
                lenc,
                command_ptr,
                &mut lenr,
                response
            )
        );

        let body = server.requests.next().unwrap().body;
        let json: Value = serde_json::from_str(&body).unwrap();

        assert_eq!(*json.get("dad").unwrap(), json!(dad));
        assert_eq!(*json.get("sad").unwrap(), json!(sad));
        assert_eq!(
            *json.get("command").unwrap(),
            json!(BASE64.encode(&command))
        );
        assert_eq!(*json.get("lenc").unwrap(), json!(lenc));
        assert_eq!(*json.get("lenr").unwrap(), json!(lenr));
    }

    #[test]
    fn response_is_mapped_to_parameter() {
        let server = test_server::new(0, |_| {
            HttpResponse::Ok()
                .body(r#"{"dad":39,"sad":63,"lenr":2,"response":"kAA=","responseCode":0}"#)
        });
        env::set_var("K2_BASE_URL", server.url());

        let (_, command, lenc, response, mut lenr, mut dad, mut sad, ctn, pn) = rand_params();
        MAP.lock().insert(ctn, pn);

        data(ctn, &mut dad, &mut sad, lenc, command, &mut lenr, response);

        assert_eq!(dad, 39);
        assert_eq!(sad, 63);
        assert_eq!(2, lenr);

        let slice = unsafe { slice::from_raw_parts(response, lenr as usize) };
        assert_eq!([144, 0], slice);
    }

    #[test]
    fn returns_err_htsi_if_server_response_is_not_200() {
        let server = test_server::new(0, |_| HttpResponse::BadRequest().into());
        env::set_var("K2_BASE_URL", server.url());

        let (_, command, lenc, response, mut lenr, mut dad, mut sad, ctn, pn) = rand_params();
        MAP.lock().insert(ctn, pn);

        assert_eq!(
            -128,
            data(ctn, &mut dad, &mut sad, lenc, command, &mut lenr, response,)
        );
    }

    #[test]
    fn returns_err_htsi_if_server_response_not_contains_response_struct_as_json() {
        let server = test_server::new(0, |_| HttpResponse::Ok().body("hello world"));
        env::set_var("K2_BASE_URL", server.url());

        let (_, command, lenc, response, mut lenr, mut dad, mut sad, ctn, pn) = rand_params();
        MAP.lock().insert(ctn, pn);

        assert_eq!(
            -128,
            data(ctn, &mut dad, &mut sad, lenc, command, &mut lenr, response,)
        );
    }

    #[test]
    fn returns_response_status_from_valid_json_response_struct() {
        let server = test_server::new(0, |_| {
            HttpResponse::Ok()
                .body(r#"{"dad":1,"sad":1,"lenr":1,"response":"a=","responseCode":-11}"#)
        });
        env::set_var("K2_BASE_URL", server.url());

        let (_, command, lenc, response, mut lenr, mut dad, mut sad, ctn, pn) = rand_params();
        MAP.lock().insert(ctn, pn);

        assert_eq!(
            -11,
            data(ctn, &mut dad, &mut sad, lenc, command, &mut lenr, response,)
        );
    }

    fn rand_params() -> (Vec<u8>, *const u8, u16, *mut u8, u16, u8, u8, u16, u16) {
        let mut command = vec![0; rand::random::<u16>() as usize];
        for x in command.iter_mut() {
            *x = rand::random::<u8>()
        }

        let command_ptr: *const u8 = command.as_ptr();
        let lenc: u16 = command.len() as u16;

        let mut response: [u8; MAX as usize] = [rand::random::<u8>(); MAX as usize];
        let response_ptr: *mut u8 = response.as_mut_ptr();
        let lenr: u16 = response.len() as u16;

        let dad = rand::random::<u8>();
        let sad = rand::random::<u8>();

        let ctn = rand::random::<u16>();
        let pn = rand::random::<u16>();

        (
            command,
            command_ptr,
            lenc,
            response_ptr,
            lenr,
            dad,
            sad,
            ctn,
            pn,
        )
    }
}
