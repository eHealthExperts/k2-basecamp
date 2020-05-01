#[cfg(windows)]
extern crate winres;

#[cfg(windows)]
fn main() {
    let res = winres::WindowsResource::new();
    let _ = res.compile();
}

#[cfg(not(windows))]
fn main() {}
