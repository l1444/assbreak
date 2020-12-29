#[cfg(target_os = "windows")]
extern crate winres;

#[cfg(target_os = "linux")]
use std::process::Command;

#[cfg(target_os = "windows")]
fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("assbreak.ico");
    res.compile().unwrap();
}

#[cfg(target_os = "macos")]
fn main() {

}

#[cfg(target_os = "linux")]
fn main() {
    Command::new("sh")
        .arg("cargo pkgbuild")
        .output()
        .expect("error");
    Command::new("sh")
        .arg("makepkg")
        .output()
        .expect("failed to compile");
}