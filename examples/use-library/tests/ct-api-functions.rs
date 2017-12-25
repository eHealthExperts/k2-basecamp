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
    let server = test_server::serve(Some(String::from("127.0.0.1:65432")));
    env::set_var("K2_BASE_URL", "http://127.0.0.1:65432");

    match Library::new(LIB_PATH) {
        Ok(lib) => {
            println!("before load init");
            let init: Symbol<unsafe extern "system" fn(u16, u16) -> i8> =
                unsafe { lib.get(b"CT_init").unwrap() };
            println!("before load data");
            let data: Symbol<
                unsafe extern "system" fn(u16, *mut u8, *mut u8, u16, *const u8, *mut u16, *mut u8)
                    -> i8,
            > = unsafe { lib.get(b"CT_data").unwrap() };
            println!("before load close");
            let close: Symbol<unsafe extern "system" fn(u16) -> i8> =
                unsafe { lib.get(b"CT_close").unwrap() };

            let ctn = rand::random::<u16>();
            println!("ctn: {}", ctn);
            let pn = rand::random::<u16>();
            println!("pn: {}", pn);
            let mut dad = rand::random::<u8>();
            println!("dad: {}", dad);
            let mut sad = rand::random::<u8>();
            println!("sad: {}", sad);

            let commands: [u8; 1] = [rand::random::<u8>(); 1];
            let commands_ptr: *const u8 = &commands[0];
            let lenc: u16 = commands.len() as u16;

            println!("command: {:?}", commands);
            println!("lenc: {}", lenc);

            let mut response: [u8; MAX as usize] = [rand::random::<u8>(); MAX as usize];
            let response_ptr: *mut u8 = &mut response[0];
            let mut lenr: u16 = response.len() as u16;

            println!("lenr: {}", lenr);

            unsafe {
                println!("before init server");
                server.reply().status(hyper::Ok).body("0");

                println!("before init");
                assert_eq!(0, init(ctn, pn));

                println!("before data server");
                server.reply().status(hyper::Ok).body(
                    "{\"dad\":1,\"sad\":1,\"lenr\":5,\"response\":\"AQIDBAU=\",\"responseCode\":0}",
                );

                println!("before data");
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

                println!("before close server");
                server.reply().status(hyper::Ok).body("0");

                println!("before close");
                assert_eq!(0, close(ctn));
            }
        }
        _ => assert!(false, format!("loading library from {}", LIB_PATH)),
    }
}
