use std::fs::File;
use std::io;
use std::io::{BufRead, Read, Write};
use std::path::Path;

pub fn read_input() -> String {
    loop {
        let mut v = String::new();
        let _ = io::stdout().flush();
        match io::stdin().read_line(&mut v) {
            Ok(_) => {
                if !String::from(&v).is_empty() {
                    break no_enter(v);
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

pub fn no_enter(str: String) -> String {
    return str.replace("\r", "").replace("\n", "")
}