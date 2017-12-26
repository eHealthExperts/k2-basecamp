extern crate libloading;
extern crate rand;
extern crate test_server;

use libloading::{Library, Symbol};
use std::{env, str};
use std::u16::MAX;
use test_server::hyper;

#[cfg(target_os = "windows")]
const LIB_PATH: &str = "../../target/debug/ctehxk2.dll";
#[cfg(target_os = "linux")]
const LIB_PATH: &str = "../../target/debug/libctehxk2.so";
#[cfg(target_os = "macos")]
const LIB_PATH: &str = "../../target/debug/libctehxk2.dylib";

#[test]
fn has_ct_api_functions() {
    unsafe {
        match Library::new(LIB_PATH) {
            Ok(lib) => {
                let init: Symbol<unsafe extern "system" fn(u16, u16) -> i8> =
                    lib.get(b"CT_init").unwrap();

                let data: Symbol<
                    unsafe extern "system" fn(
                        u16,
                        *mut u8,
                        *mut u8,
                        u16,
                        *const u8,
                        *mut u16,
                        *mut u8,
                    ) -> i8,
                > = lib.get(b"CT_data").unwrap();

                let close: Symbol<unsafe extern "system" fn(u16) -> i8> =
                    lib.get(b"CT_close").unwrap();

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

                let server = test_server::serve(Some(String::from("127.0.0.1:65432")));
                env::set_var("K2_BASE_URL", "http://127.0.0.1:65432");

                server.reply().status(hyper::Ok).body("0");
                assert_eq!(0, init(ctn, pn));

                // response for data
                server.reply().status(hyper::Ok).body(
                    "{\"dad\":1,\"sad\":1,\"lenr\":5,\"response\":\"AQIDBAU=\",\"responseCode\":0}",
                );
                assert_eq!(
                    0,
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

                server.reply().status(hyper::Ok).body("0");
                assert_eq!(0, close(ctn));
            }
            _ => assert!(false, format!("loading library from {}", LIB_PATH)),
        }
    }
}
