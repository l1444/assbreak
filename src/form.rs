//local
use super::func::*;
use super::config::*;
use crate::brute_force::email::brute_mail;
use crate::brute_force::website::brute_website;
use colored::Colorize;

pub fn input_text_password_submit() {
    // website for bruteforce
    print!("{}", "[~] The site you want to try brute-force : ".green());
    let website = read_input();

    // username to crack
    print!("{}", "[~] Enter the username to brute-force : ".green());
    let username = read_input();

    // dictionary_password
    print!("{}", "[~] Enter the path of the password dictionary : ".green());
    let path_file = read_input();

    // keyword
    print!("{}", "[~] A word to filter passwords (optional): ".green());
    let key = read_input();

    print!("{}", "[~] Do you want to see chrome in the middle of a brute force operation (yes or no) ? ".green());
    let chrome = read_input();
    if chrome == "yes" {
        println!("{}", "[!] The program will show you chrome and its brute-force operation".bright_yellow());
        default_config(website, username, path_file, key, true);
    } else {
        println!("{}", "[!] You will not see chrome however you will be able to see the logs.".bright_yellow());
        default_config(website, username, path_file, key, false);
    }
}


//input
pub fn input_email_password_submit() {
    // website for bruteforce
    print!("{}", "[~] The site you want to try brute-force : ".green());
    let website = read_input();

    // username to crack
    print!("{}", "[~] Enter the username to brute-force : ".green());
    let username = read_input();

    // dictionary_password
    print!("{}", "[~] Enter the path of the password dictionary : ".green());
    let path_file = read_input();

    // keyword
    print!("{}", "[~] A word to filter passwords (optional): ".green());
    let key = read_input();

    print!("{}", "[~] Do you want to see chrome in the middle of a brute force operation (yes or no) ? ".green());
    let chrome = read_input();

    if chrome == "yes" {
        println!("{}", "[!] The program will show you chrome and its brute-force operation".bright_yellow());
        config_field_email(website, username, path_file, key, true);
    } else {
        println!("{}", "[!] You will not see chrome however you will be able to see the logs.".bright_yellow());
        config_field_email(website, username, path_file, key, false);
    }
}
pub fn input_text_password_button() {
    // website for bruteforce
    print!("{}", "[~] The site you want to try brute-force : ".green());
    let website = read_input();

    // username to crack
    print!("{}", "[~] Enter the username to brute-force : ".green());
    let username = read_input();

    // dictionary_password
    print!("{}", "[~] Enter the path of the password dictionary : ".green());
    let path_file = read_input();

    // keyword
    print!("{}", "[~] A word to filter passwords (optional): ".green());
    let key = read_input();

    print!("{}", "[~] Do you want to see chrome in the middle of a brute force operation (yes or no) ? ".green());
    let chrome = read_input();
    if chrome == "yes" {
        println!("{}", "[!] The program will show you chrome and its brute-force operation".bright_yellow());
        config_field_button(website, username, path_file, key,true);
    } else {
        println!("{}", "[!] You will not see chrome however you will be able to see the logs.".bright_yellow());
        config_field_button(website, username, path_file, key,false);

    }
}
pub fn input_email_password_button() {
    // website for bruteforce
    print!("{}", "[~] The site you want to try brute-force : ".green());
    let website = read_input();

    // username to crack
    print!("{}", "[~] Enter the username to brute-force : ".green());
    let username = read_input();

    // dictionary_password
    print!("{}", "[~] Enter the path of the password dictionary : ".green());
    let path_file = read_input();

    // keyword
    print!("{}", "[~] A word to filter passwords (optional): ".green());
    let key = read_input();

    print!("{}", "[~] Do you want to see chrome in the middle of a brute force operation (yes or no) ? ".green());
    let chrome = read_input();
    if chrome == "yes" {
        println!("{}", "[!] The program will show you chrome and its brute-force operation".bright_yellow());
        config_field_email_button(website, username, path_file, key, true);
    } else {
        println!("{}", "[!] You will not see chrome however you will be able to see the logs.".bright_yellow());
        config_field_email_button(website, username, path_file, key, false);
    }

}
pub fn input_button() {
    // website for bruteforce
    print!("{}", "[~] The site you want to try brute-force : ".green());
    let website = read_input();

    // input button
    print!("{}", "[~] Enter the button (login) selector : ".green());
    let btn = read_input();

    // username to crack
    print!("{}", "[~] Enter the username to brute-force : ".green());
    let username = read_input();

    // dictionary_password
    print!("{}", "[~] Enter the path of the password dictionary : ".green());
    let path_file = read_input();

    // keyword
    print!("{}", "[~] A word to filter passwords (optional): ".green());
    let key = read_input();

    print!("{}", "[~] Do you want to see chrome in the middle of a brute force operation (yes or no) ? ".green());
    let chrome = read_input();
    if chrome == "yes" {
        println!("{}", "[!] The program will show you chrome and its brute-force operation".bright_yellow());
        config_button(website, username, path_file ,btn, key, true);
    } else {
        println!("{}", "[!] You will not see chrome however you will be able to see the logs.".bright_yellow());
        config_button(website, username, path_file ,btn, key, false);
    }

}
pub fn input_email_button() {
    // website for bruteforce
    print!("{}", "[~] The site you want to try brute-force : ".green());
    let website = read_input();

    // input button
    print!("{}", "[~] Enter the button (login) selector : ".green());
    let btn = read_input();

    // username to crack
    print!("{}", "[~] Enter the username to brute-force : ".green());
    let username = read_input();

    // dictionary_password
    print!("{}", "[~] Enter the path of the password dictionary : ".green());
    let path_file = read_input();

    // keyword
    print!("{}", "[~] A word to filter passwords (optional): ".green());
    let key = read_input();

    print!("{}", "[~] Do you want to see chrome in the middle of a brute force operation (yes or no) ? ".green());
    let chrome = read_input();
    if chrome == "yes" {
        println!("{}", "[!] The program will show you chrome and its brute-force operation".bright_yellow());
        config_email_button(website, username, path_file ,btn, key, true);
    } else {
        println!("{}", "[!] You will not see chrome however you will be able to see the logs.".bright_yellow());
        config_email_button(website, username, path_file ,btn, key, false);
    }

}
pub fn default() {
    // website for bruteforce
    print!("{}", "[~] The site you want to try brute-force : ".green());
    let website = read_input();

    // input username
    print!("{}", "[~] Enter the username selector : ".green());
    let username_input = read_input();

    // input password
    print!("{}", "[~] Enter the password selector : ".green());
    let password_input = read_input();

    // input button
    print!("{}", "[~] Enter the button (login) selector : ".green());
    let form_button = read_input();

    // username to crack
    print!("{}", "[~] Enter the username to brute-force : ".green());
    let username = read_input();

    // dictionary_password
    print!("{}", "[~] Enter the path of the password dictionary : ".green());
    let path_file = read_input();

    // keyword
    print!("{}", "[~] A word to filter passwords (optional): ".green());
    let key = read_input();

    print!("{}", "[~] Do you want to see chrome in the middle of a brute force operation (yes or no) ? ".green());
    let chrome = read_input();

    if chrome == "yes" {
        println!("{}", "[!] The program will show you chrome and its brute-force operation".bright_yellow());
        brute_website(website, username_input, password_input, form_button, username, path_file, key, true);

    } else {
        println!("{}", "[!] You will not see chrome however you will be able to see the logs.".bright_yellow());
        brute_website(website, username_input, password_input, form_button, username, path_file, key, false);

    }


}

//email
pub fn brute_email() {
    print!("{}", "[~] Enter the imap address of the service with which you want to brute-force : ".green());
    let imap = read_input();

    print!("{}", "Enter a brute-force email address : ".green());
    let username = read_input();

    // dictionary_password
    print!("{}", "[~] Enter the path of the password dictionary : ".green());
    let path_file = read_input();

    // keyword
    print!("{}", "[~] A word to filter passwords (optional): ".green());
    let key = read_input();

    brute_mail(username, path_file, key,imap);
}
pub fn brute_email_outlook() {
    print!("{}", "[~] Enter the outlook address to brute-force : ".green());
    let username = read_input();

    // dictionary_password
    print!("{}", "[~] Enter the path of the password dictionary : ".green());
    let path_file = read_input();

    // keyword
    print!("{}", "[~] A word to filter passwords (optional): ".green());
    let key = read_input();

    brute_mail(username, path_file, key,String::from("imap-mail.outlook.com"));
}
pub fn brute_email_icloud() {
    print!("{}", "[~] Enter the brute-force icloud address : ".green());
    let username = read_input();

    // dictionary_password
    print!("{}", "[~] Enter the path of the password dictionary : ".green());
    let path_file = read_input();

    // keyword
    print!("{}", "[~] A word to filter passwords (optional): ".green());
    let key = read_input();

    brute_mail(username, path_file, key,String::from("imap.mail.me.com"));
}
pub fn brute_email_yahoo() {
    print!("{}", "[~] Enter the brute-force yahoo address : ".green());
    let username = read_input();

    // dictionary_password
    print!("{}", "[~] Enter the path of the password dictionary : ".green());
    let path_file = read_input();

    // keyword
    print!("{}", "[~] A word to filter passwords (optional): ".green());
    let key = read_input();

    brute_mail(username, path_file, key, String::from("imap.mail.yahoo.com"));
}

pub fn query() {

    // keyword
    print!("{}", "[~] Enter the passwords corresponding to a keyword : ".green());
    let key = read_input();

    // dictionary_password
    print!("{}", "[~] Enter the path of the password dictionary : ".green());
    let path_file = read_input();

    search_by_keyword(key, path_file);
}