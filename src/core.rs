use crate::{
    //controllers::*,
    views::{
        color_console::*,
        input::*
    },
    models::{
        selenium::*,
        imap::*
    }
};
use super::{
    configuration,
    Config
};

use std::{
    thread,
    sync::mpsc::channel
};


use threadpool::ThreadPool;
///
/// Operating systems compatible with assbreak. 
/// 
pub enum OS {
    Windows(u8),
    MacOS,
    Linux
}
///
/// Brute-force systems available on assbreak.
/// 
pub enum TypeBrute {
    Selenium,
    Imap
    // j'espere bientot d'autre système de brute-force
}

pub enum ConfigBruteImap {
    // imap
    ImapOutlook,
    ImapYahoo,
    ImapiCloud,
    ImapPersonnalize, 
    None
}
pub enum ConfigBruteSelenium {
    SeleniumTextButton,
    SeleniumEmailButton,
    SeleniumTextInput,
    SeleniumEmailInput,
    SeleniumPersonnalize,
    None
}


pub fn core(nb_threads: usize, type_brute: TypeBrute, config_brute_imap: ConfigBruteImap, config_brute_selenium: ConfigBruteSelenium) {
    let n_jobs: i32 = nb_threads as i32;
    let pool = ThreadPool::new(nb_threads);
    let (tx, rx) = channel();

    let config = configuration();

    let mut c: Config = Config::new();
        // get all data in the json
        match std::fs::read_to_string(&config) {
            Ok(string) => {
                let str: &str = &string[..];
                let data = str;
                c = serde_json::from_str(data).unwrap();
            }
            Err(_) => {
                configuration();
            }
        }

    let mut _addr = String::new();
    let mut _username = String::new();
    let mut _dico = String::from(&c.dictionary);
    let mut _filter = String::new();

    // only selenium
    let mut _input = String::new();
    let mut _input_pass = String::new();
    let mut _btn = String::new(); 


    match type_brute {
        TypeBrute::Selenium => {
            _addr = Input::new("[~] What website did you want brute-force? : ");
            _username = Input::new("[~] Which user do you want brute-force? : ");
            _dico = Input::new(&format!("Do you want to change the password dictionary? (default: {}): ", &_dico));
            _filter = Input::new("[~] A word to filter passwords (optional): ");

            match config_brute_selenium {
                ConfigBruteSelenium::SeleniumPersonnalize => {
                    _input = Input::new("");
                    _input_pass = Input::new("");
                    _btn  = Input::new("");
                },
                ConfigBruteSelenium::SeleniumTextButton => {
                    _input = "input[type=text]".to_owned();
                    _input_pass = "input[type=password]".to_owned();
                    _btn = "button".to_owned();
                },
                ConfigBruteSelenium::SeleniumTextInput => {
                    _input = "input[type=text]".to_owned();
                    _input_pass = "input[type=password]".to_owned();
                    _btn = "input[type=submit]".to_owned();
                },
                ConfigBruteSelenium::SeleniumEmailButton => {
                    _input = "input[type=email]".to_owned();
                    _input_pass = "input[type=password]".to_owned();
                    _btn = "button".to_owned();
                },
                ConfigBruteSelenium::SeleniumEmailInput => {
                    _input = "input[type=email]".to_owned();
                    _input_pass = "input[type=password]".to_owned();
                    _btn = "input[type=submit]".to_owned();
                },
                ConfigBruteSelenium::None => (),
            }
        }
        TypeBrute::Imap => {
            match config_brute_imap {
                ConfigBruteImap::ImapOutlook => {
                    _addr = String::from("imap-mail.outlook.com");
                    _username = Input::new("[~] What email address do you want brute-force? : ");
                    _dico = Input::new(&format!("[~] Do you want to change the password dictionary? (default: {}): ", &_dico));
                    _filter = Input::new("[~] A word to filter passwords (optional): ");
                }
                ConfigBruteImap::ImapYahoo => {
                    _addr = String::from("imap.mail.yahoo.com");
                    _username = Input::new("[~] What email address do you want brute-force? : ");
                    _dico = Input::new(&format!("[~] Do you want to change the password dictionary? (default: {}): ", &_dico));
                    _filter = Input::new("[~] A word to filter passwords (optional): ");
                }
                ConfigBruteImap::ImapiCloud => {
                    _addr = String::from("imap.mail.me.com");
                    _username = Input::new("[~] What email address do you want brute-force? : ");
                    _dico = Input::new(&format!("[~] Do you want to change the password dictionary? (default: {}): ", &_dico));
                    _filter = Input::new("[~] A word to filter passwords (optional): ");
                }
                ConfigBruteImap::ImapPersonnalize => {
                    _addr = Input::new("[~] What is the imap address of the email address you want brute-force? : ");
                    _username = Input::new("[~] What email address do you want brute-force? : ");
                    _dico = Input::new(&format!("[~] Do you want to change the password dictionary? (default: {}): ", &_dico));
                    _filter = Input::new("[~] A word to filter passwords (optional): ");
                }
                ConfigBruteImap::None => ()
            }
        }
        _ => ColorConsole::colorized("An error has occurred".to_owned(), Color::Red),
    };

    loop {
        // sender and receiver data
        for _ in 0..n_jobs {
            let tx = tx.clone();
            pool.execute(move|| {
                tx.send(true).unwrap();
            });
            println!("{}", &rx.recv().unwrap());
        }
        thread::sleep(std::time::Duration::from_secs(1));
    }
}
