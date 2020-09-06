#[macro_use]
extern crate const_cstr;
#[macro_use]
extern crate serial_test;

#[cfg(target_os = "windows")]
const LIB_PATH: &str = "./target/debug/ctehxk2.dll";
#[cfg(target_os = "linux")]
const LIB_PATH: &str = "./target/debug/libctehxk2.so";
#[cfg(target_os = "macos")]
const LIB_PATH: &str = "./target/debug/libctehxk2.dylib";

#[test]
#[serial]
fn init() -> anyhow::Result<()> {
    let lib = dlopen::raw::Library::open(LIB_PATH)?;

    let init: unsafe extern "system" fn(u16, u16) -> i8 =
        unsafe { lib.symbol_cstr(const_cstr!("CT_init").as_cstr()) }?;

    let ctn = rand::random::<u16>();
    let pn = rand::random::<u16>();

    assert_eq!(-128, unsafe { init(ctn, pn) });

    Ok(())
}

#[test]
#[serial]
fn close() -> anyhow::Result<()> {
    let lib = dlopen::raw::Library::open(LIB_PATH)?;

    let close: unsafe extern "system" fn(u16) -> i8 =
        unsafe { lib.symbol_cstr(const_cstr!("CT_close").as_cstr()) }?;

    let ctn = rand::random::<u16>();

    assert_eq!(-1, unsafe { close(ctn) });

    Ok(())
}

#[test]
#[serial]

fn data_null_pointer() -> anyhow::Result<()> {
    let lib = dlopen::raw::Library::open(LIB_PATH)?;

    let data: unsafe extern "system" fn(
        u16,
        *mut u8,
        *mut u8,
        u16,
        *const u8,
        *mut u16,
        *mut u8,
    ) -> i8 = unsafe { lib.symbol_cstr(const_cstr!("CT_data").as_cstr()) }?;

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
    assert_eq!(-128, unsafe {
        data(
            ctn,
            dad_null,
            &mut sad,
            lenc,
            commands_ptr,
            &mut lenr,
            response_ptr,
        )
    });

    let sad_null: *mut u8 = std::ptr::null_mut();
    assert_eq!(-128, unsafe {
        data(
            ctn,
            &mut dad,
            sad_null,
            lenc,
            commands_ptr,
            &mut lenr,
            response_ptr,
        )
    });

    let lenr_null: *mut u16 = std::ptr::null_mut();
    assert_eq!(-128, unsafe {
        data(
            ctn,
            &mut dad,
            &mut sad,
            lenc,
            commands_ptr,
            lenr_null,
            response_ptr,
        )
    });

    let response_null: *mut u8 = std::ptr::null_mut();
    assert_eq!(-128, unsafe {
        data(
            ctn,
            &mut dad,
            &mut sad,
            lenc,
            commands_ptr,
            &mut lenr,
            response_null,
        )
    });

    Ok(())
}
