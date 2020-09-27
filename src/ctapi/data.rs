use crate::ctapi::MAP;
use crate::{http, Status, CONFIG};
use data_encoding::{BASE64, HEXLOWER};
use std::slice;

#[allow(non_snake_case)]
#[cfg_attr(test, derive(Debug, PartialEq))]
#[derive(Deserialize)]
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
) -> anyhow::Result<Status> {
    if let Some(ctn_from_cfg) = CONFIG.read().ctn {
        debug!("Use ctn '{}' from configuration", ctn_from_cfg);
        ctn = ctn_from_cfg;
    }

    if !MAP.read().contains_key(&ctn) {
        error!("Card terminal has not been opened.");
        return Ok(Status::ERR_INVALID);
    }

    let pn = match MAP.read().get(&ctn) {
        None => return Err(format_err!("Failed to extract pn for given ctn!")),
        Some(pn) => *pn,
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
    let response = http::request(&path, Some(json))?;

    match serde_json::from_str::<Response>(&response) {
        Err(why) => {
            debug!("{}", why);
            Err(format_err!("Unexpected server response found in body!"))
        }
        Ok(json) => {
            let status = Status::from(json.status);
            if let Status::OK = status {
                let decoded = match BASE64.decode(&json.response.into_bytes()) {
                    Ok(content) => {
                        debug!("Decoded response field: {:?}", HEXLOWER.encode(&content));
                        content
                    }
                    Err(why) => {
                        debug!("{}", why);
                        return Err(format_err!("Failed to extract response."));
                    }
                };

                for (place, element) in safe_response.iter_mut().zip(decoded.iter()) {
                    *place = *element;
                }

                *safe_dad = json.dad;
                *safe_sad = json.sad;
                *safe_lenr = json.lenr;
            }
            Ok(status)
        }
    }
}

#[cfg(test)]
mod tests {

    use super::{data, Response};
    use crate::{ctapi::MAP, Status};
    use data_encoding::BASE64;
    use serde_json::{self, json, Value};
    use std::{
        env::{remove_var, set_var},
        slice,
        u16::MAX,
    };
    use wiremock::{matchers, Mock, MockServer, ResponseTemplate};

    #[test]
    fn deserialize_response() {
        let res: Result<Response, _> = serde_json::from_str(
            "{\"dad\":1,\"sad\":1,\"lenr\":5,\"response\":\"AQIDBAU=\",\"responseCode\":0}",
        );

        assert!(res.is_ok());
        assert_eq!(
            res.unwrap(),
            Response {
                dad: 1,
                sad: 1,
                lenr: 5,
                response: "AQIDBAU=".to_string(),
                status: 0
            }
        );
    }

    #[test]
    fn returns_err_invalid_if_terminal_closed() {
        let (_, command, lenc, response, mut lenr, mut dad, mut sad, ctn, _) = rand_params();

        assert_eq!(
            Some(Status::ERR_INVALID),
            data(ctn, &mut dad, &mut sad, lenc, command, &mut lenr, response,).ok()
        );
    }

    #[test]
    #[serial]
    fn returns_err_if_no_server() {
        set_var("K2_BASE_URL", "http://127.0.0.1:65432");
        crate::tests::init_config_clear_map();

        let (_, command, lenc, response, mut lenr, mut dad, mut sad, ctn, pn) = rand_params();
        let _ = MAP.write().insert(ctn, pn);

        assert!(data(ctn, &mut dad, &mut sad, lenc, command, &mut lenr, response).is_err());

        remove_var("K2_BASE_URL");
    }

    #[async_std::test]
    #[serial]
    async fn use_ctn_and_pn_in_request_path() {
        let (_, command, lenc, response, mut lenr, mut dad, mut sad, ctn, pn) = rand_params();
        let _ = MAP.write().insert(ctn, pn);

        let mock_server = MockServer::start().await;
        Mock::given(matchers::path(format!("/ct_data/{}/{}", ctn, pn)))
            .respond_with(ResponseTemplate::new(400))
            .mount(&mock_server)
            .await;
        set_var("K2_BASE_URL", mock_server.uri());

        crate::tests::init_config_clear_map();

        let _ = data(ctn, &mut dad, &mut sad, lenc, command, &mut lenr, response);

        remove_var("K2_BASE_URL");
    }

    #[async_std::test]
    #[serial]
    async fn post_body_contains_parameter() -> Result<(), anyhow::Error> {
        let mock_server = MockServer::start().await;
        Mock::given(matchers::any())
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;
        set_var("K2_BASE_URL", mock_server.uri());

        crate::tests::init_config_clear_map();

        let (command, command_ptr, lenc, response, mut lenr, mut dad, mut sad, ctn, pn) =
            rand_params();
        let _ = MAP.write().insert(ctn, pn);

        let _ = data(
            ctn,
            &mut dad,
            &mut sad,
            lenc,
            command_ptr,
            &mut lenr,
            response,
        );

        match &mock_server.received_requests().await {
            Some(requests) if requests.last().is_some() => {
                let request = requests.last().unwrap();
                let request_body = serde_json::from_slice::<Value>(&request.body).unwrap();

                assert_eq!(
                    request_body,
                    json!({
                        "command": BASE64.encode(&command),
                        "dad": dad,
                        "lenc": lenc,
                        "lenr": lenr,
                        "sad": sad,

                    })
                );
            }
            _ => bail!("Missing requests"),
        }

        remove_var("K2_BASE_URL");

        Ok(())
    }

    #[async_std::test]
    #[serial]
    async fn response_is_mapped_to_parameter() {
        let mock_server = MockServer::start().await;
        Mock::given(matchers::any())
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "dad":39,
                "sad":63,
                "lenr":2,
                "response":"kAA=",
                "responseCode":0
            })))
            .mount(&mock_server)
            .await;
        set_var("K2_BASE_URL", mock_server.uri());

        crate::tests::init_config_clear_map();

        let (_, command, lenc, response, mut lenr, mut dad, mut sad, ctn, pn) = rand_params();
        let _ = MAP.write().insert(ctn, pn);

        let _ = data(ctn, &mut dad, &mut sad, lenc, command, &mut lenr, response);

        assert_eq!(dad, 39);
        assert_eq!(sad, 63);
        assert_eq!(2, lenr);

        let slice = unsafe { slice::from_raw_parts(response, lenr as usize) };
        assert_eq!([144, 0], slice);

        remove_var("K2_BASE_URL");
    }

    #[async_std::test]
    #[serial]
    #[should_panic(expected = "Failed to extract response.")]
    async fn response_with_failure_response_field() {
        let mock_server = MockServer::start().await;
        Mock::given(matchers::any())
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "dad":39,
                "sad":63,
                "lenr":2,
                "response":"0123456789",
                "responseCode":0
            })))
            .mount(&mock_server)
            .await;
        set_var("K2_BASE_URL", mock_server.uri());

        crate::tests::init_config_clear_map();

        let (_, command, lenc, response, mut lenr, mut dad, mut sad, ctn, pn) = rand_params();
        let _ = MAP.write().insert(ctn, pn);

        let res = data(ctn, &mut dad, &mut sad, lenc, command, &mut lenr, response);

        remove_var("K2_BASE_URL");

        let _status = res.unwrap();
    }

    #[async_std::test]
    #[serial]
    async fn returns_err_if_server_response_is_not_200() {
        let mock_server = MockServer::start().await;
        Mock::given(matchers::any())
            .respond_with(ResponseTemplate::new(400))
            .mount(&mock_server)
            .await;
        set_var("K2_BASE_URL", mock_server.uri());

        crate::tests::init_config_clear_map();

        let (_, command, lenc, response, mut lenr, mut dad, mut sad, ctn, pn) = rand_params();
        let _ = MAP.write().insert(ctn, pn);

        assert!(data(ctn, &mut dad, &mut sad, lenc, command, &mut lenr, response,).is_err());

        remove_var("K2_BASE_URL");
    }

    #[async_std::test]
    #[serial]
    async fn returns_err_if_server_response_not_contains_response_struct_as_json() {
        let mock_server = MockServer::start().await;
        Mock::given(matchers::any())
            .respond_with(ResponseTemplate::new(200).set_body_string("hello world"))
            .mount(&mock_server)
            .await;
        set_var("K2_BASE_URL", mock_server.uri());

        crate::tests::init_config_clear_map();

        let (_, command, lenc, response, mut lenr, mut dad, mut sad, ctn, pn) = rand_params();
        let _ = MAP.write().insert(ctn, pn);

        assert!(data(ctn, &mut dad, &mut sad, lenc, command, &mut lenr, response,).is_err());

        remove_var("K2_BASE_URL");
    }

    #[async_std::test]
    #[serial]
    async fn returns_response_status_from_valid_json_response_struct() {
        let mock_server = MockServer::start().await;
        Mock::given(matchers::any())
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "dad":1,
                "sad":1,
                "lenr":1,
                "response":"a=",
                "responseCode":-11
            })))
            .mount(&mock_server)
            .await;
        set_var("K2_BASE_URL", mock_server.uri());

        crate::tests::init_config_clear_map();

        let (_, command, lenc, response, mut lenr, mut dad, mut sad, ctn, pn) = rand_params();
        let _ = MAP.write().insert(ctn, pn);

        assert_eq!(
            Some(Status::ERR_MEMORY),
            data(ctn, &mut dad, &mut sad, lenc, command, &mut lenr, response,).ok()
        );

        remove_var("K2_BASE_URL");
    }

    #[async_std::test]
    #[serial]
    async fn use_ctn_and_pn_from_config() {
        let (_, command, lenc, response, mut lenr, mut dad, mut sad, ctn, pn) = rand_params();

        let mock_server = MockServer::start().await;
        Mock::given(matchers::path(format!("/ct_data/{}/{}", ctn, pn)))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;
        set_var("K2_BASE_URL", mock_server.uri());

        set_var("K2_CTN", format!("{}", ctn));
        set_var("K2_PN", format!("{}", pn));
        crate::tests::init_config_clear_map();

        let _ = MAP.write().insert(ctn, pn);

        let unused_ctn = rand::random::<u16>();

        let _ = data(
            unused_ctn, &mut dad, &mut sad, lenc, command, &mut lenr, response,
        );

        remove_var("K2_BASE_URL");
        remove_var("K2_CTN");
        remove_var("K2_PN");
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
