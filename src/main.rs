//local
use func::*;
use view::header;

//std
use std::io;
use std::io::Write;
use std::process::exit;


// mod
mod brute;
mod func;
mod config;
mod view;
mod form;

///
/// Coded by L14ms1 <l14ms1@outlook.fr>
/// last update 23:01 - 28/11/2020
/// LICENSE MIT
///
///


fn main() {
    header();
    print!("[~] What configuration do you want to use ? : ");
    let mut commands = String::new();
    let _ = io::stdout().flush();
    // dispatcher
    match io::stdin().read_line(&mut commands) {
        Ok(_) => {
            match String::as_str(&filter(commands)) {
                "1" => {
                    form::input_text_password_submit();
                }
                "2" => {
                    form::input_email_password_submit();
                }
                "3" => {
                    form::input_text_password_button();
                }
                "4" => {
                    form::input_email_password_button()
                }
                "5" => {
                    form::default();
                }
                "6" => {
                    form::brute_email_outlook();
                }
                "7" => {
                    form::brute_email_icloud();
                }
                "8" => {
                    form::brute_email_yahoo();
                }
                "9" => {
                    form::brute_email();
                }
                _ => {}
            }
        }
        Err(e) => {
            println!("An error has occurred : {}", e);
        }
    }
    pause();
    exit(0);
}

