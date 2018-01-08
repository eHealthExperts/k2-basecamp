use self::super::MAP;
use self::super::super::{http, Status};
use base64::{decode, encode};
use serde_json;
use std::slice;

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
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
    use serde_json;
    use std::env;
    use std::slice;
    use std::str;
    use std::u16::MAX;
    use test_server::{self, http};

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
        env::set_var("K2_BASE_URL", "http://127.0.0.1:65432");

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
        let server = test_server::serve(None);
        server.reply().status(http::StatusCode::BAD_REQUEST);
        env::set_var("K2_BASE_URL", format!("http://{}", &server.addr()));

        let (commands_ptr, lenc, response_ptr, mut lenr, mut dad, mut sad, ctn, pn) = rand_params();
        MAP.lock().insert(ctn, pn);

        data(
            ctn,
            &mut dad,
            &mut sad,
            lenc,
            commands_ptr,
            &mut lenr,
            response_ptr,
        );

        let (parts, _body) = server.request().unwrap().into_parts();
        assert_eq!(parts.uri, *format!("/ct_data/{}/{}", ctn, pn));
    }

    #[test]
    fn post_body_contains_parameter() {
        let server = test_server::serve(None);
        server.reply().status(http::StatusCode::BAD_REQUEST);
        env::set_var("K2_BASE_URL", format!("http://{}", &server.addr()));

        let (command, lenc, response, mut lenr, mut dad, mut sad, ctn, pn) = rand_params();
        let slice = unsafe { slice::from_raw_parts(command, lenc as usize) };
        MAP.lock().insert(ctn, pn);

        assert_eq!(
            -128,
            data(ctn, &mut dad, &mut sad, lenc, command, &mut lenr, response)
        );

        #[derive(Deserialize, Debug)]
        struct Json {
            dad: u8,
            sad: u8,
            lenc: u16,
            command: String,
            lenr: u16,
        }

        let (_parts, body) = server.request().unwrap().into_parts();
        let json: Json = serde_json::from_str(&body).unwrap();

        assert_eq!(dad, json.dad);
        assert_eq!(sad, json.sad);
        assert_eq!(encode(slice), json.command);
        assert_eq!(lenc, json.lenc);
        assert_eq!(lenr, json.lenr);
    }

    #[test]
    fn response_is_mapped_to_parameter() {
        let server = test_server::serve(None);
        server.reply().status(http::StatusCode::OK).body(
            "{\"dad\":39,\"sad\":63,\"lenr\":5,\"response\":\"AQIDBAU=\",\"responseCode\":0}",
        );

        env::set_var("K2_BASE_URL", format!("http://{}", &server.addr()));

        let (command, lenc, response, mut lenr, mut dad, mut sad, ctn, pn) = rand_params();
        MAP.lock().insert(ctn, pn);

        data(ctn, &mut dad, &mut sad, lenc, command, &mut lenr, response);

        assert_eq!(dad, 39);
        assert_eq!(sad, 63);
        assert_eq!(5, lenr);

        let slice = unsafe { slice::from_raw_parts(response, lenr as usize) };
        assert_eq!([1, 2, 3, 4, 5], slice);
    }

    #[test]
    fn returns_err_htsi_if_server_response_is_not_200() {
        let server = test_server::serve(None);
        server.reply().status(http::StatusCode::BAD_REQUEST);
        env::set_var("K2_BASE_URL", format!("http://{}", &server.addr()));

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
        );
    }

    #[test]
    fn returns_err_htsi_if_server_response_not_contains_response_struct_as_json() {
        let server = test_server::serve(None);
        server
            .reply()
            .status(http::StatusCode::OK)
            .body("hello world");
        env::set_var("K2_BASE_URL", format!("http://{}", &server.addr()));

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
        );
    }

    #[test]
    fn returns_response_status_from_valid_json_response_struct() {
        let server = test_server::serve(None);
        server
            .reply()
            .status(http::StatusCode::OK)
            .body("{\"dad\":1,\"sad\":1,\"lenr\":1,\"response\":\"a\",\"responseCode\":-11}");
        env::set_var("K2_BASE_URL", format!("http://{}", &server.addr()));

        let (commands_ptr, lenc, response_ptr, mut lenr, mut dad, mut sad, ctn, pn) = rand_params();
        MAP.lock().insert(ctn, pn);

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
    }
}
