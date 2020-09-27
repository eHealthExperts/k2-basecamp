#[macro_use]
extern crate serial_test;

use dlopen::raw::Library;
use serde_json::json;
use std::{env, str, u16::MAX};
use wiremock::{matchers, Mock, MockServer, ResponseTemplate};

#[cfg(target_os = "windows")]
const LIB_PATH: &str = "./target/debug/ctehxk2.dll";
#[cfg(target_os = "linux")]
const LIB_PATH: &str = "./target/debug/libctehxk2.so";
#[cfg(target_os = "macos")]
const LIB_PATH: &str = "./target/debug/libctehxk2.dylib";

#[async_std::test]
#[serial]
async fn init_data_close() -> anyhow::Result<()> {
    let mock_server = MockServer::start().await;
    env::set_var("K2_BASE_URL", mock_server.uri());

    let data_mock = Mock::given(matchers::path_regex("^/ct_data")).respond_with(
        ResponseTemplate::new(200).set_body_json(json!({
            "dad":1,
            "sad":1,
            "lenr":5,
            "response":"AQIDBAU=",
            "responseCode":0
        })),
    );
    let ok_mock = Mock::given(matchers::path_regex("^/ct_init|/ct_close"))
        .respond_with(ResponseTemplate::new(200).set_body_json(0));

    mock_server.register(data_mock).await;
    mock_server.register(ok_mock).await;

    let lib = Library::open(LIB_PATH)?;

    let init: unsafe extern "system" fn(u16, u16) -> i8 = unsafe { lib.symbol("CT_init") }?;

    let data: unsafe extern "system" fn(
        u16,
        *mut u8,
        *mut u8,
        u16,
        *const u8,
        *mut u16,
        *mut u8,
    ) -> i8 = unsafe { lib.symbol("CT_data") }?;

    let close: unsafe extern "system" fn(u16) -> i8 = unsafe { lib.symbol("CT_close") }?;

    let ctn = rand::random::<u16>();
    let pn = rand::random::<u16>();
    let mut dad = rand::random::<u8>();
    let mut sad = rand::random::<u8>();

    let commands: [u8; 1] = [rand::random::<u8>(); 1];
    let commands_ptr: *const u8 = &commands[0];
    let lenc: u16 = commands.len() as u16;

    let mut response: [u8; MAX as usize] = [rand::random::<u8>(); MAX as usize];
    let response_ptr: *mut u8 = &mut response[0];
    let mut lenr: u16 = response.len() as u16;

    assert_eq!(0, unsafe { init(ctn, pn) });
    assert_eq!(0, unsafe {
        data(
            ctn,
            &mut dad,
            &mut sad,
            lenc,
            commands_ptr,
            &mut lenr,
            response_ptr,
        )
    });
    assert_eq!(0, unsafe { close(ctn) });

    env::remove_var("K2_BASE_URL");
    Ok(())
}
