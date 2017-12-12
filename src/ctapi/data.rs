extern crate serde_json;

use self::super::{Status, MAP};
use self::super::super::http;
use base64::{decode, encode};
use std::slice;

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct Response {
    dad: u8,
    sad: u8,
    lenr: u16,
    response: String,
    #[serde(rename = "responseCode")] status: i8,
}

pub fn data(
    ctn: u16,
    dad: *mut u8,
    sad: *mut u8,
    lenc: u16,
    command: *const u8,
    lenr: *mut u16,
    response: *mut u8,
) -> Status {
    if !MAP.lock().contains_key(&ctn) {
        error!("Card terminal has not been opened.");
        return Status::ErrInvalid;
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

    let pn = MAP.lock().get(&ctn).unwrap().clone();
    let path = format!("ct_data/{}/{}", ctn, pn);

    let response = http::request(&path, Some(json));
    let res = match response {
        Ok(response) => response,
        Err(why) => {
            error!("{}", why);
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
            error!("Failed to parse server response data. {}", why);
            return Status::ErrHtsi;
        }
    };

    let status: Status = json.status.into();
    match status {
        Status::OK => {
            *safe_dad = json.dad;
            *safe_sad = json.sad;
            *safe_lenr = json.lenr;

            let decoded = decode(&json.response).unwrap();
            debug!("Decoded response field {:?}", decoded);

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

    use super::data;
    use super::super::MAP;
    use base64::encode;
    use rand;
    use rouille::{self, Response};
    use std::slice;
    use std::u16::MAX;

    fn rand_params() -> (*const u8, u16, *mut u8, u16, u8, u8, u16, u16) {
        let commands: [u8; 1] = [rand::random::<u8>(); 1];
        let commands_ptr: *const u8 = &commands[0];
        let lenc: u16 = commands.len() as u16;

        let mut response: [u8; MAX as usize] = [rand::random::<u8>(); MAX as usize];
        let response_ptr: *mut u8 = &mut response[0];
        let lenr: u16 = response.len() as u16;

        let dad = rand::random::<u8>();
        let sad = rand::random::<u8>();

        let ctn = rand::random::<u16>();
        let pn = rand::random::<u16>();

        (commands_ptr, lenc, response_ptr, lenr, dad, sad, ctn, pn)
    }

    #[test]
    fn returns_err_invalid_if_terminal_closed() {
        let (commands_ptr, lenc, response_ptr, mut lenr, mut dad, mut sad, ctn, _pn) =
            rand_params();

        assert_eq!(
            -1,
            data(
                ctn,
                &mut dad,
                &mut sad,
                lenc,
                commands_ptr,
                &mut lenr,
                response_ptr,
            )
        )
    }

    #[test]
    fn returns_err_htsi_if_no_server() {
        let (commands_ptr, lenc, response_ptr, mut lenr, mut dad, mut sad, ctn, pn) = rand_params();

        MAP.lock().insert(ctn, pn);

        assert_eq!(
            -128,
            data(
                ctn,
                &mut dad,
                &mut sad,
                lenc,
                commands_ptr,
                &mut lenr,
                response_ptr,
            )
        )
    }

    #[test]
    fn use_ctn_and_pn_in_request_path() {
        let (commands_ptr, lenc, response_ptr, mut lenr, mut dad, mut sad, ctn, pn) = rand_params();

        MAP.lock().insert(ctn, pn);

        let shutdown = test_server!((request: &Request) {
            assert_eq!(request.url(), format!("/ct_data/{}/{}", ctn, pn));

            Response::empty_404()
        });

        data(
            ctn,
            &mut dad,
            &mut sad,
            lenc,
            commands_ptr,
            &mut lenr,
            response_ptr,
        );

        // kill server thread
        let _ = shutdown.send(());
    }

    #[test]
    fn post_body_contains_parameter() {
        let (command, lenc, response, mut lenr, mut dad, mut sad, ctn, pn) = rand_params();

        let slice = unsafe { slice::from_raw_parts(command, lenc as usize) };

        MAP.lock().insert(ctn, pn);

        let shutdown = test_server!((request: &Request) {
            assert_eq!(request.url(), format!("/ct_data/{}/{}", ctn, pn));

            #[derive(RustcDecodable)]
            struct Json {
                dad: u8,
                sad: u8,
                lenc: u16,
                command: String,
                lenr: u16,
            }

            let json: Json = rouille::input::json_input(request).unwrap();

            assert_eq!(dad, json.dad);
            assert_eq!(sad, json.sad);
            assert_eq!(encode(slice), json.command);
            assert_eq!(lenc, json.lenc);
            assert_eq!(lenr, json.lenr);

            Response::empty_404()
        });

        assert_eq!(
            -128,
            data(ctn, &mut dad, &mut sad, lenc, command, &mut lenr, response)
        );

        // kill server thread
        let _ = shutdown.send(());
    }

    #[test]
    fn response_is_mapped_to_parameter() {
        let (command, lenc, response, mut lenr, mut dad, mut sad, ctn, pn) = rand_params();

        MAP.lock().insert(ctn, pn);

        let shutdown = test_server!((request: &Request) {
            assert_eq!(request.url(), format!("/ct_data/{}/{}", ctn, pn));

            #[derive(RustcDecodable)]
            struct JsonRequest {
                dad: u8,
                sad: u8,
                lenc: u16,
                command: String,
                lenr: u16,
            }

            let json: JsonRequest = rouille::input::json_input(request).unwrap();

            #[allow(non_snake_case)]
            #[derive(RustcEncodable)]
            struct JsonResponse {
                dad: u8,
                sad: u8,
                lenr: u16,
                response: String,
                responseCode: u16,
            }

            let response = Response::json(&JsonResponse {
                dad: json.sad,
                sad: json.dad,
                lenr: 5,
                response: String::from("AQIDBAU="),
                responseCode: 0
            });

            response
        });

        let c_sad = sad.clone();
        let c_dad = dad.clone();

        let status = data(ctn, &mut dad, &mut sad, lenc, command, &mut lenr, response);

        assert_eq!(0, status);
        assert_eq!(dad, c_sad);
        assert_eq!(sad, c_dad);
        assert_eq!(5, lenr);

        let slice = unsafe { slice::from_raw_parts(response, lenr as usize) };
        assert_eq!([1, 2, 3, 4, 5], slice);

        // kill server thread
        let _ = shutdown.send(());
    }

    #[test]
    fn returns_err_htsi_if_server_response_is_not_200() {
        let (commands_ptr, lenc, response_ptr, mut lenr, mut dad, mut sad, ctn, pn) = rand_params();

        MAP.lock().insert(ctn, pn);

        let shutdown = test_server!((request: &Request) {
            Response::empty_404()
        });

        assert_eq!(
            -128,
            data(
                ctn,
                &mut dad,
                &mut sad,
                lenc,
                commands_ptr,
                &mut lenr,
                response_ptr,
            )
        );

        // kill server thread
        let _ = shutdown.send(());
    }

    #[test]
    fn returns_err_htsi_if_server_response_not_contains_response_struct_as_json() {
        let (commands_ptr, lenc, response_ptr, mut lenr, mut dad, mut sad, ctn, pn) = rand_params();

        MAP.lock().insert(ctn, pn);

        let shutdown = test_server!((request: &Request) {
            Response::text("hello world")
        });

        assert_eq!(
            -128,
            data(
                ctn,
                &mut dad,
                &mut sad,
                lenc,
                commands_ptr,
                &mut lenr,
                response_ptr,
            )
        );

        // kill server thread
        let _ = shutdown.send(());
    }

    #[test]
    fn returns_response_status_from_valid_json_response_struct() {
        let (commands_ptr, lenc, response_ptr, mut lenr, mut dad, mut sad, ctn, pn) = rand_params();

        MAP.lock().insert(ctn, pn);

        let shutdown = test_server!((request: &Request) {
            Response::text(r#"{"dad":1,"sad":1,"lenr":1,"response":"a","responseCode":-11}"#)
        });

        assert_eq!(
            -11,
            data(
                ctn,
                &mut dad,
                &mut sad,
                lenc,
                commands_ptr,
                &mut lenr,
                response_ptr,
            )
        );

        // kill server thread
        let _ = shutdown.send(());
    }
}
