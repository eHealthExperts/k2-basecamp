#[macro_use]
extern crate const_cstr;
extern crate dlopen;
extern crate rand;
extern crate test_server;

use dlopen::raw::Library;
use std::u16::MAX;
use std::{env, str};
use test_server::http::StatusCode;

#[cfg(target_os = "windows")]
const LIB_PATH: &str = "../../target/debug/ctehxk2.dll";
#[cfg(target_os = "linux")]
const LIB_PATH: &str = "../../target/debug/libctehxk2.so";
#[cfg(target_os = "macos")]
const LIB_PATH: &str = "../../target/debug/libctehxk2.dylib";

#[test]
fn has_ct_api_functions() {
    let lib = Library::open(LIB_PATH).expect("Could not open library");

    let init: unsafe extern "system" fn(u16, u16) -> i8 =
        unsafe { lib.symbol_cstr(const_cstr!("CT_init").as_cstr()) }.unwrap();

    let data: unsafe extern "system" fn(
        u16,
        *mut u8,
        *mut u8,
        u16,
        *const u8,
        *mut u16,
        *mut u8,
    ) -> i8 = unsafe { lib.symbol_cstr(const_cstr!("CT_data").as_cstr()) }.unwrap();

    let close: unsafe extern "system" fn(u16) -> i8 =
        unsafe { lib.symbol_cstr(const_cstr!("CT_close").as_cstr()) }.unwrap();

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

    let server = test_server::serve(Some("127.0.0.1:65432"));
    env::set_var("K2_BASE_URL", "http://127.0.0.1:65432");

    server.reply().status(StatusCode::OK).body("0");
    assert_eq!(0, unsafe { init(ctn, pn) });

    server
        .reply()
        .status(StatusCode::OK)
        .body("{\"dad\":1,\"sad\":1,\"lenr\":5,\"response\":\"AQIDBAU=\",\"responseCode\":0}");
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

    server.reply().status(StatusCode::OK).body("0");
    assert_eq!(0, unsafe { close(ctn) });
}
