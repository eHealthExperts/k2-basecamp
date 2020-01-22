#[cfg(windows)]
extern crate winres;

#[cfg(windows)]
fn main() {
    let res = winres::WindowsResource::new();
    res.compile();
}

#[cfg(not(windows))]
fn main() {}
