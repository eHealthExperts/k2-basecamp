use super::*;
use crate::{ctapi::MAP, Settings, CONFIG};
use std::collections::HashMap;

pub fn init_config_clear_map() {
    let mut config_guard = CONFIG.write();
    *config_guard = Settings::init().unwrap();
    drop(config_guard);

    let mut map_guard = MAP.write();
    *map_guard = HashMap::new();
    drop(map_guard);
}

#[test]
#[serial]
fn init() -> anyhow::Result<()> {
    let server = test_server::new("127.0.0.1:0", || {
        test_server::HttpResponse::Ok().body("hello world")
    })?;
    std::env::set_var("K2_BASE_URL", server.url());
    init_config_clear_map();

    let ctn = rand::random::<u16>();
    let pn = rand::random::<u16>();

    assert_eq!(-128, CT_init(ctn, pn));

    Ok(())
}

#[test]
#[serial]
fn close_with_error() -> anyhow::Result<()> {
    let server = test_server::new("127.0.0.1:0", || {
        test_server::HttpResponse::Ok().body("hello world")
    })?;
    std::env::set_var("K2_BASE_URL", server.url());
    init_config_clear_map();

    let ctn = rand::random::<u16>();
    let pn = rand::random::<u16>();

    let _ = MAP.write().insert(ctn, pn);

    assert_eq!(-128, CT_close(ctn));

    Ok(())
}

#[test]
#[serial]
fn data_with_error() -> anyhow::Result<()> {
    let server = test_server::new("127.0.0.1:0", || {
        test_server::HttpResponse::Ok().body("hello world")
    })?;
    std::env::set_var("K2_BASE_URL", server.url());

    init_config_clear_map();

    let ctn = rand::random::<u16>();
    let pn = rand::random::<u16>();

    let mut dad = rand::random::<u8>();
    let mut sad = rand::random::<u8>();

    let commands: [u8; 1] = [rand::random::<u8>(); 1];
    let commands_ptr: *const u8 = &commands[0];
    let lenc: u16 = rand::random::<u16>();

    let mut response: [u8; std::u16::MAX as usize] = [rand::random::<u8>(); std::u16::MAX as usize];
    let response_ptr: *mut u8 = &mut response[0];
    let mut lenr: u16 = rand::random::<u16>();

    let _ = MAP.write().insert(ctn, pn);

    assert_eq!(
        -128,
        CT_data(
            ctn,
            &mut dad,
            &mut sad,
            lenc,
            commands_ptr,
            &mut lenr,
            response_ptr,
        )
    );

    Ok(())
}

#[test]
#[serial]
fn data_null_pointer() -> anyhow::Result<()> {
    init_config_clear_map();

    let ctn = rand::random::<u16>();
    let mut dad = rand::random::<u8>();
    let mut sad = rand::random::<u8>();

    let commands: [u8; 1] = [rand::random::<u8>(); 1];
    let commands_ptr: *const u8 = &commands[0];
    let lenc: u16 = rand::random::<u16>();

    let mut response: [u8; std::u16::MAX as usize] = [rand::random::<u8>(); std::u16::MAX as usize];
    let response_ptr: *mut u8 = &mut response[0];
    let mut lenr: u16 = rand::random::<u16>();

    let dad_null: *mut u8 = std::ptr::null_mut();
    assert_eq!(
        -128,
        CT_data(
            ctn,
            dad_null,
            &mut sad,
            lenc,
            commands_ptr,
            &mut lenr,
            response_ptr,
        )
    );

    let sad_null: *mut u8 = std::ptr::null_mut();
    assert_eq!(
        -128,
        CT_data(
            ctn,
            &mut dad,
            sad_null,
            lenc,
            commands_ptr,
            &mut lenr,
            response_ptr,
        )
    );

    let lenr_null: *mut u16 = std::ptr::null_mut();
    assert_eq!(
        -128,
        CT_data(
            ctn,
            &mut dad,
            &mut sad,
            lenc,
            commands_ptr,
            lenr_null,
            response_ptr,
        )
    );

    let response_null: *mut u8 = std::ptr::null_mut();
    assert_eq!(
        -128,
        CT_data(
            ctn,
            &mut dad,
            &mut sad,
            lenc,
            commands_ptr,
            &mut lenr,
            response_null,
        )
    );

    Ok(())
}
