#[cfg(target_os = "windows")]
extern crate winres;

#[cfg(target_os = "windows")]
fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("res/assbreak.ico");
    res.compile().unwrap();
}

#[cfg(target_os = "macos")]
fn main() {}

#[cfg(target_os = "linux")]
fn main() {}
