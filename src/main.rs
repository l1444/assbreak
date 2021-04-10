//std
use std::{env::temp_dir, io};
use std::io::Write;
use std::process::exit;
use std::path::PathBuf;
use std::path::Path;
use std::fs;
use std::env;

// mod
mod brute_force;
mod config;
mod func;
mod view;
mod models;
mod views;
mod core;
mod error;

// extern
use console::{Term};
use dialoguer::{Select, theme::ColorfulTheme};
use directories::BaseDirs;

use serde::{Deserialize, Serialize};
use serde_json::{Result};

use models::discord::*;
use models::selenium::*;

use views::assbreak::*;
use views::alert::*;
use views::color_console::*;
use views::input::*;
use views::filter::*;

use models::webrequests::*;

use crate::{
    core::*, 
    error::ASSBREAK_ERROR
};

///
/// The version of application
/// 
static VERSION_APP: &str = "0.2.1";
///
/// The link has a actual version of application and not application version
/// 
static VERSION_URL: &str = "https://raw.githubusercontent.com/L14ms111/assbreak/master/VERSION";

///
/// # ASSBREAK, *When I recover a password from a stranger, it's like I piss off this stranger* :)
///
/// Coded by **L14ms1 <l14ms1@protonmail.com>**;
///
/// last update 16:58 - 05/02/2021
///
/// LICENSE MPL-2.0
///
/// Author is french, not english !
/// 
/// The entire code is done by a newbie to Rust, so that explains why the code sucks.
/// 
/// 
fn main() {

    // The presentation of assbreak
    let term = Term::stdout();
    term.clear_screen().unwrap();
    term.set_title("assbreak");
    assbreak::new();
    ColorConsole::colorized("************************************************************************************\n*                                                                           v0.2.1 *".to_string(), Color::Green);
    ColorConsole::colorized("*  Author : L14                                                                    *".to_string(), Color::Green);
    ColorConsole::colorized("*  Github : https://github.com/L14ms111/assbreak                                   *".to_string(), Color::Green);
    ColorConsole::colorized("*  Discord: https://discord.gg/km8jggx43c                                          *\n*                                                               coding for pa$$ion *".to_string(), Color::Green);
    ColorConsole::colorized("************************************************************************************".to_string(), Color::Green);
    // RPC
    if WebRequests::new("http://httpbin.org/get").send_requests().get_status() != "" {
        let d_rpc = DiscordInfos::new("assbreak", "a tool for brute-force website mail address.", "assbreak", "");
        d_rpc.start_rpc();
    }

    // First, if the configuration file was not created, we created it!
    let config = configuration();

    // the parts, the most importants of assbreak, because he has all dispatchers
    controller(config);
}

///
/// This essential function for the use of assbreak, makes it possible to see the commands carried out and to be able to dispatch, each time the command is good, it is redirected to a controller allowing the task to continue.
/// 
fn controller(path_json: String)  -> Result<()> {

    // this variable store all line of commands put in assbreak
    let mut history_commands: Vec<String> = vec![];

    loop {
        let mut c: Config = Config::new();
        // get all data in the json
        match fs::read_to_string(&path_json) {
            Ok(string) => {
                let str: &str = &string[..];
                let data = str;
                c = serde_json::from_str(data).unwrap();
            }
            Err(_) => {
                configuration();
                continue
            }
        }
        // indicated the username
        let user = String::from(c.username);

        let mut commands = String::new();
        ColorConsole::colorized(format!("$[assbreak@{}]-> ", user), Color::DarkGreen);
        let console = console::Term::stdout();

        
        let _ = io::stdout().flush();
        // Using the Input::new() function does not work in this kind of situation, and other functions from assbreak.
        match console.read_line() {
            Ok(commands) => {
                match String::as_str(&commands.replace("\n", "")) {
                    "$check-all" => {
                        if WebRequests::new("https://github.com").check_connection() == true {
                            ColorConsole::colorized("[~] You have internet connection :)".to_string(), Color::Green);
                        } else {
                            ColorConsole::colorized("[x] You are offline.".to_string(), Color::Red);
                        }

                        if SeleniumConfig::check_selenium() == true {
                            ColorConsole::colorized("\r[~] Selenium has been started.".to_string(), Color::Green);
                        } else {
                            ColorConsole::colorized("\r[x] Selenium has not been started.".to_string(), Color::Red);
                        }
                    },
                    // to check selenium is if working or not
                    "$check-selenium" => {
                        if SeleniumConfig::check_selenium() == true {
                            ColorConsole::colorized("\r[!] Selenium is detected and ready to use :), 
                            \rif it doesn't work, please check you have version 3.141.59.".to_string(), Color::Green);
                        } else {
                            ColorConsole::colorized("\r[x] assbreak does not detect Selenium start :/
                            \rIf you want to know how to install it, 
                            \rI invite you to go to this link: https://www.selenium.dev/downloads/ and download Selenium Grid under version 3.141.59.
                            \rIf you already have it installed, 
                            \rplease start it with Java, and double clicking or else go to the command prompt and type: 'java -jar selenium.jar'".to_string(), Color::Red);
                        }
                    },
                    // check connection
                    "$check-connection" => {
                        if WebRequests::new("https://github.com").check_connection() == true {
                            ColorConsole::colorized("[!] Yeh! You have internet connection, you will be able to brute force without worries.".to_string(), Color::Green);
                        } else {
                            ColorConsole::colorized("[x] Damn ... You don't have a connection, you can brute-force only locally.".to_string(), Color::Red);
                        }
                    },
                    // clear all of line of commands
                    "$clear" => {
                        let term = Term::stdout();
                        term.clear_screen().unwrap();
                    },
                    // To modify the configuration.json file
                    "$config" => {
                        ColorConsole::colorized("***************************************".to_owned(), Color::Green);
                        ColorConsole::colorized(format!("* Username : {}", &user), Color::Green);
                        ColorConsole::colorized(format!("* OS : {}", c.os), Color::Green);
                        ColorConsole::colorized(format!("* Threads (default) : {}", c.threads), Color::Green);
                        ColorConsole::colorized(format!("* Selenium Path : {}", c.dictionary), Color::Green);
                        ColorConsole::colorized(format!("* Path dictionnary : {}", c.dictionary), Color::Green);
                        ColorConsole::colorized(format!("* Path config : {}", &path_json), Color::Green);
                        ColorConsole::colorized("***************************************".to_owned(), Color::Green);
                    },
                    "$delete-config" => {
                        fs::remove_file(&path_json).unwrap();
                        fs::remove_file(c.dictionary).unwrap();
                    },
                    // to get help
                    "$help" => {
                        ColorConsole::colorized("*************************************** HELP ***************************************".to_string(), Color::Green);
                        ColorConsole::colorized("*                                                                                  *".to_string(), Color::Green);
                        ColorConsole::colorized("* $check-connection - Only check if you havve a connection.                     *".to_string(), Color::Green);
                        ColorConsole::colorized("* $check-selenium - Only check if selenium has been started.                       *".to_string(), Color::Green);
                        ColorConsole::colorized("* $help - To get help.                                                             *".to_string(), Color::Green);
                        ColorConsole::colorized("* $imap - Allowing to brute-force an email address.                                *".to_string(), Color::Green);
                        ColorConsole::colorized("* $quit - Exit the program.                                                        *".to_string(), Color::Green);
                        ColorConsole::colorized("* $selenium - To be able to brute-force a website.                                 *".to_string(), Color::Green);
                        ColorConsole::colorized("* $update - To see if there is not a new update. (but don't install it)            *".to_string(), Color::Green);
                        ColorConsole::colorized("*                                                                                  *".to_string(), Color::Green);
                        ColorConsole::colorized("************************************************************************************".to_string(), Color::Green);
                    },
                    // for using imap
                    "$imap" => {
                        println!("");
                        ColorConsole::colorized("********************** IMAP (brute force of an email address) **********************".to_string(), Color::Green);
                        ColorConsole::colorized("*                                                                           (Beta) *".to_string(), Color::Green);
                        ColorConsole::colorized("* Configuration 1 : Outlook (imap-mail.outlook.com)                                *".to_string(), Color::Green);
                        ColorConsole::colorized("* Configuration 2 : Yahoo! (imap.mail.yahoo.com)                                   *".to_string(), Color::Green);
                        ColorConsole::colorized("* Configuration 3 : iCloud (imap.mail.me.com)                                      *".to_string(), Color::Green);
                        ColorConsole::colorized("* Configuration 4 : AOL (imap.aol.com)                                             *".to_string(), Color::Green);
                        ColorConsole::colorized("* Configuration 5 : Customize (imap address)                                       *".to_string(), Color::Green);
                        ColorConsole::colorized("*                                                                                  *".to_string(), Color::Green);
                        ColorConsole::colorized("************************************************************************************".to_string(), Color::Green);
                        println!("");
                    },
                    // quit the program
                    "$quit" => {
                        let x = Input::new("[~] Are you sure you want to quit assbreak? (y or n) : ");
                        if x == "y" {
                            ColorConsole::colorized("see you next time :)".to_string(), Color::Green);
                            exit(0);
                        } else {
                            ColorConsole::colorized("I knew I had to insist that you don't go :o".to_string(), Color::Green);
                        }
                    },
                    // for using selenium
                    "$selenium" => {
                        let mut commands = String::new();
                        let mut thread: usize = 1;
                        let threads = Input::new(&format!("[~] How many threads do you want (default: {})? ", &c.threads));
                        let th: usize = match threads.parse::<usize>() {
                            Ok(uint) => {
                                thread = uint;
                                uint
                            },
                            Err(err) => {
                                ASSBREAK_ERROR::new(ASSBREAK_ERROR::Err1);
                                1
                            }
                        };
                        println!("");
                        ColorConsole::colorized("************************************* SELENIUM *************************************".to_string(), Color::Green);
                        ColorConsole::colorized("*                                                                          (Alpha) *".to_string(), Color::Green);
                        ColorConsole::colorized("* Configuration 1 : input[type=text], input[type=password], button                 *".to_string(), Color::Green);
                        ColorConsole::colorized("* Configuration 2 : input[type=email], input[type=password], button                *".to_string(), Color::Green);
                        ColorConsole::colorized("* Configuration 3 : input[type=text], input[type=password], input[type=submit]     *".to_string(), Color::Green);
                        ColorConsole::colorized("* Configuration 4 : input[type=email], input[type=password], input[type=submit]    *".to_string(), Color::Green);
                        ColorConsole::colorized("* Configuration 5 : Customize (using CSS attributes to indicate an element)        *".to_string(), Color::Green);
                        ColorConsole::colorized("*                                                                                  *".to_string(), Color::Green);
                        ColorConsole::colorized("************************************************************************************".to_string(), Color::Green);
                        println!("");
                        let mut commands = String::new();
                        ColorConsole::colorized("[~] What configuration do you want(ex: 2)? ".to_owned(), Color::DarkGreen);
                        let _ = io::stdout().flush();
                        match io::stdin().read_line(&mut commands) {
                            Ok(_) => {
                                match String::as_str(&Filter::new(commands, true, true).filter_now()) {
                                    "1" => {
                                        core(thread, TypeBrute::Selenium, ConfigBruteImap::None, ConfigBruteSelenium::SeleniumTextButton);
                                    }, 
                                    "2" => {
                                        core(thread, TypeBrute::Selenium, ConfigBruteImap::None, ConfigBruteSelenium::SeleniumEmailButton);
                                    },
                                    "3" => {
                                        core(thread, TypeBrute::Selenium, ConfigBruteImap::None, ConfigBruteSelenium::SeleniumTextInput);
                                    },
                                    "4" => {
                                        core(thread, TypeBrute::Selenium, ConfigBruteImap::None, ConfigBruteSelenium::SeleniumEmailInput);
                                    },
                                    "5" => {
                                        core(thread, TypeBrute::Selenium, ConfigBruteImap::None, ConfigBruteSelenium::SeleniumPersonnalize);
                                    },
                                    _ => ASSBREAK_ERROR::new(ASSBREAK_ERROR::Err2),
                                }
                            }
                            _ => ASSBREAK_ERROR::new(ASSBREAK_ERROR::Err3)
                        }
                    },
                    // check if update has here or not
                    "$update" => {
                        let req = WebRequests::new(VERSION_URL).send_requests();
                        let version_actual = Filter::new(req.get_body(), true, true).filter_now();
                        if version_actual != "" {
                            if version_actual != VERSION_APP {
                                ColorConsole::colorized(format!("[!] A new version is available, it is available on the github. (v{} -> v{})", &VERSION_APP, &version_actual), Color::Yellow);
                            } else {
                                ColorConsole::colorized(format!("[!] No new version has been released :)"), Color::Green);
                            }  
                        } else {
                            ColorConsole::colorized(format!("[x] Please check your connection, otherwise the server storing the current version of assbreak is not working :/"), Color::Red);
                        }
                    },
                    _ => ASSBREAK_ERROR::new(ASSBREAK_ERROR::Err6)
                }
                history_commands.push(commands.replace("\n", ""));
            }
            // a error in the line of commands
            Err(_) => ASSBREAK_ERROR::new(ASSBREAK_ERROR::Err3)
        }

    }

    Ok(())

}

#[derive(Serialize, Deserialize)]
struct Config {
    version_assbreak: String,
    username: String,
    os: String,
    packet: String,
    dictionary: String,
    threads: usize
}

impl Config {

    pub fn new() -> Config {
        Config {
            version_assbreak: String::new(),
            username: String::new(),
            os: String::new(),
            packet: String::new(),
            dictionary: String::new(),
            threads: 1
        }
    }

}

fn configuration() -> String {
    let mut folder_config = String::new();
    let mut conf_file = String::new();
    if let Some(dir) = BaseDirs::new() {
        folder_config =  dir.data_dir().to_str().unwrap().to_owned();
        folder_config.push_str("/assbreak/");
        conf_file = format!("{}.conf", &folder_config);
	println!("{}", conf_file);
	
        if Path::new(&conf_file).exists() == false {
            let _ = fs::remove_dir_all(&folder_config);                
            match fs::create_dir(&folder_config) {
                Ok(_) => {
                    let mut os = String::new();
                    let mut packet = String::new();
                    if cfg!(target_os = "windows") {
                        os = "Windows".to_string();
                    } else if cfg!(target_os = "macos") {
                        os = "macOS".to_string();
                    } else if cfg!(target_os = "linux") {
                        os = "Linux".to_string();
                        ColorConsole::colorized("[!] Who is your packet manager? \n".to_string(), Color::DarkGreen);                            let _ = io::stdout().flush();
                        match Select::with_theme(&ColorfulTheme::default()).item("APT (ubuntu)").item("Pacman (Arch)").item("rpm (Red Hat, Fedora, Mageia, etc...)").item("Other").interact() {
                            Ok(choose) => {
                                match choose {
                                        0 => {
                                            packet = "APT".to_string();
                                        },
                                        1 => {
                                            packet = "Pacman".to_string();
                                        },
                                        2 => {
                                            packet = "RPM".to_string();
                                        },
                                        3 => {
                                            packet = "Other".to_string();
                                        },
                                        _ => {
                                            packet = "".to_string();
                                        }
                                    }
                            },
                            Err(_) => {
                                ColorConsole::colorized("[!] An error has occurred.".to_string(), Color::Yellow);
                            },
                            _ => {
                                ColorConsole::colorized("This command line does not exist !".to_string(), Color::Red);
                            }
                        }
                    }

                    if packet == "" {
                        packet = "Other".to_owned();
                    }

                    print!("{}", &packet);

                    let mut username = Input::new("\n[~] What username would you like? ");
                    if username == "" {
                        username = "Guest".to_owned();
                    }
                    let mut path_dico = Input::new("[~] Where is your dictionary password? (ex : C:/../dictionary.txt) ");
                    if path_dico == "" {
                        if WebRequests::new("https://raw.githubusercontent.com").check_connection() == true {
                            ColorConsole::colorized("[!] I download a password dictionary just for you <3".to_owned(), Color::Green);
                            let dico = WebRequests::new("https://raw.githubusercontent.com/L14ms111/assbreak/bonus/passlist.txt").send_requests().get_body();
                            let path = format!("{}passlist.txt",folder_config);
                            let _ = fs::File::create(&path).expect("Anywhere there can be an error and the error is undetermined.");
                            let _ = fs::write(&path, dico).expect("Anywhere there can be an error and the error is undetermined.");
                            path_dico = path;
                        } else {
                            path_dico = "nothing".to_owned();
                        }
                    }
                    let nb_threads = Input::new("[~] During your password brute force, how many threads do you want (the more there are, the faster it will be, so it will take more resources on your computer)? ");
                    // 
                    let mut threads: usize = if nb_threads == "" {
                        1
                    } else {
                        match nb_threads.parse::<usize>() {
                            Ok(_) => {
                                if nb_threads.parse::<usize>().unwrap() == 0 {
                                    1
                                } else {
                                    nb_threads.parse::<usize>().unwrap()
                                }
                            },
                            Err(_) => {
                                1
                            }, 
                        }
                    };

                    let config = Config {
                        version_assbreak: String::from(VERSION_APP),
                        username,
                        os,
                        packet,
                        dictionary: path_dico,
                        threads
                    };            
                    match serde_json::to_string(&config) {
                        Ok(json) => {
                            ColorConsole::colorized("[!] The creation of the configuration file was successful, you can go to assbreak now :)".to_string(), Color::Green);
                            let _file = fs::File::create(&conf_file).expect("Anywhere there can be an error and the error is undetermined.");
                            let _write_file = fs::write(&conf_file, json).expect("Anywhere there can be an error and the error is undetermined.");
                        }
                        Err(_) => {
                            ColorConsole::colorized("[!] The configuration file could not be created, however I advise you to restart assbreak.".to_string(), Color::Red);
                        }
                    }
                }, 
            Err(_) => {}
           }
        }
    }
    return conf_file.clone();
}

