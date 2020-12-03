// std
use std::fs::File;
use std::io;
use std::io::{BufRead, Read, Write};
use std::path::Path;
use std::process::exit;

//extern
use ureq::get;

pub fn read_input() -> String {
    loop {
        let mut v = String::new();
        let _ = io::stdout().flush();
        match io::stdin().read_line(&mut v) {
            Ok(_) => {
                if !String::from(&v).is_empty() {
                    break filter(v);
                } else {
                    continue;
                }
            }
            Err(_) => {
                println!("An error has occurred");
                continue;
            }
        }
    }
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

pub fn pause() {
    write!(io::stdout(), "[!] Press any enter to quit...").unwrap();
    io::stdout().flush().unwrap();
    let _ = io::stdin().read(&mut [0u8]).unwrap();
}

pub fn filter(str: String) -> String {
    return str.replace("\r", "").replace("\n", "");
}


pub fn verify_prerequisites() -> bool {
    let chromedriver = String::from("C:/webdrivers/chromedriver.exe");
    let selenium = String::from("C:/webdrivers/selenium.jar");
    if Path::new(chromedriver.trim()).exists() && Path::new(selenium.trim()).exists() {
        true
    } else {
        false
    }
}


pub fn verify_update(v: String, addr: String) -> bool {
    if String::from(download_string(addr)) == v {
        false
    } else {
        true
    }
}

pub fn download_string(addr: String) -> String {
    filter(ureq::get(&*addr).call().into_string().unwrap()).replace(" ", "")
}

pub fn search_by_keyword(keys: String, dictionary: String) {
    if Path::new(dictionary.trim()).exists() {
        let mut list_word = vec![String::from("")];
        if let Ok(lines) = read_lines(dictionary.trim()) {
            let mut attempt = 0;
            for line in lines {
                if let Ok(l) = line {
                    while l.contains(&keys) && l != list_word[attempt] {
                        list_word.push(l.clone());
                        println!("{}", list_word[attempt]);
                        attempt += 1;
                    }
                }
            }
        }
    } else {
        pause();
        exit(0);
    }
}

