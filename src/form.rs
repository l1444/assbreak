//local
use super::func::*;
use super::config::*;
use crate::brute_force::email::brute_mail;
use crate::brute_force::website::brute_website;

pub fn input_text_password_submit() {
    // website for bruteforce
    print!("[~] The site you want to try brute-force : ");
    let website = read_input();

    // username to crack
    print!("[~] Enter the username to brute-force : ");
    let username = read_input();

    // dictionary_password
    print!("[~] Enter the path of the password dictionary : ");
    let path_file = read_input();

    // keyword
    print!("[~] A word to filter passwords (optional): ");
    let key = read_input();

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

    default_config(website, username, path_file, key, bool_chrome);
}


//input
pub fn input_email_password_submit() {
    // website for bruteforce
    print!("[~] The site you want to try brute-force : ");
    let website = read_input();

    // username to crack
    print!("[~] Enter the username to brute-force : ");
    let username = read_input();

    // dictionary_password
    print!("[~] Enter the path of the password dictionary : ");
    let path_file = read_input();

    // keyword
    print!("[~] A word to filter passwords (optional): ");
    let key = read_input();

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

    config_field_email(website, username, path_file, key, bool_chrome);
}
pub fn input_text_password_button() {
    // website for bruteforce
    print!("[~] The site you want to try brute-force : ");
    let website = read_input();

    // username to crack
    print!("[~] Enter the username to brute-force : ");
    let username = read_input();

    // dictionary_password
    print!("[~] Enter the path of the password dictionary : ");
    let path_file = read_input();

    // keyword
    print!("[~] A word to filter passwords (optional): ");
    let key = read_input();

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

    config_field_button(website, username, path_file, key,bool_chrome);
}
pub fn input_email_password_button() {
    // website for bruteforce
    print!("[~] The site you want to try brute-force : ");
    let website = read_input();

    // username to crack
    print!("[~] Enter the username to brute-force : ");
    let username = read_input();

    // dictionary_password
    print!("[~] Enter the path of the password dictionary : ");
    let path_file = read_input();

    // keyword
    print!("[~] A word to filter passwords (optional): ");
    let key = read_input();

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

    config_field_email_button(website, username, path_file, key, bool_chrome);
}
pub fn input_button() {
    // website for bruteforce
    print!("[~] The site you want to try brute-force : ");
    let website = read_input();

    // input button
    print!("[~] Enter the button (login) selector : ");
    let btn = read_input();

    // username to crack
    print!("[~] Enter the username to brute-force : ");
    let username = read_input();

    // dictionary_password
    print!("[~] Enter the path of the password dictionary : ");
    let path_file = read_input();

    // keyword
    print!("[~] A word to filter passwords (optional): ");
    let key = read_input();

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

    config_button(website, username, path_file ,btn, key, bool_chrome);
}
pub fn input_email_button() {
    // website for bruteforce
    print!("[~] The site you want to try brute-force : ");
    let website = read_input();

    // input button
    print!("[~] Enter the button (login) selector : ");
    let btn = read_input();

    // username to crack
    print!("[~] Enter the username to brute-force : ");
    let username = read_input();

    // dictionary_password
    print!("[~] Enter the path of the password dictionary : ");
    let path_file = read_input();

    // keyword
    print!("[~] A word to filter passwords (optional): ");
    let key = read_input();

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

    config_email_button(website, username, path_file ,btn, key, bool_chrome);
}
pub fn default() {
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

    // keyword
    print!("[~] A word to filter passwords (optional): ");
    let key = read_input();

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

    brute_website(website, username_input, password_input, form_button, username, path_file, key, bool_chrome);
}

//email
pub fn brute_email() {
    print!("[~] Enter the imap address of the service with which you want to brute-force : ");
    let imap = read_input();

    print!("[~] Enter a brute-force email address : ");
    let username = read_input();

    // dictionary_password
    print!("[~] Enter the path of the password dictionary : ");
    let path_file = read_input();

    // keyword
    print!("[~] A word to filter passwords (optional): ");
    let key = read_input();

    brute_mail(username, path_file, key,imap);
}
pub fn brute_email_outlook() {
    print!("[~] Enter the outlook address to brute-force : ");
    let username = read_input();

    // dictionary_password
    print!("[~] Enter the path of the password dictionary : ");
    let path_file = read_input();

    // keyword
    print!("[~] A word to filter passwords (optional): ");
    let key = read_input();

    brute_mail(username, path_file, key,String::from("imap-mail.outlook.com"));
}
pub fn brute_email_icloud() {
    print!("[~] Enter the brute-force icloud address : ");
    let username = read_input();

    // dictionary_password
    print!("[~] Enter the path of the password dictionary : ");
    let path_file = read_input();

    // keyword
    print!("[~] A word to filter passwords (optional): ");
    let key = read_input();

    brute_mail(username, path_file, key,String::from("imap.mail.me.com"));
}
pub fn brute_email_yahoo() {
    print!("[~] Enter the brute-force yahoo address : ");
    let username = read_input();

    // dictionary_password
    print!("[~] Enter the path of the password dictionary : ");
    let path_file = read_input();

    // keyword
    print!("[~] A word to filter passwords (optional): ");
    let key = read_input();

    brute_mail(username, path_file, key, String::from("imap.mail.yahoo.com"));
}

pub fn query() {

    // keyword
    print!("[~] Enter the passwords corresponding to a keyword : ");
    let key = read_input();

    // dictionary_password
    print!("[~] Enter the path of the password dictionary : ");
    let path_file = read_input();

    search_by_keyword(key, path_file);
}