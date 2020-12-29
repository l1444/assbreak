//local
use super::func::*;
use super::config::*;
use crate::brute_force::email::brute_mail;
use crate::brute_force::website::brute_website;


pub fn input_text_password_submit() {
    // website for bruteforce
    color_terminal("[~] The site you want to try brute-force : ", "dark_green");
    let website = read_input();

    // username to crack
    color_terminal("[~] Enter the username to brute-force : ", "dark_green");
    let username = read_input();

    // dictionary_password
    color_terminal("[~] Enter the path of the password dictionary : ", "dark_green");
    let path_file = read_input();

    // keyword
    color_terminal("[~] A word to filter passwords (optional): ", "dark_green");
    let key = read_input();

    color_terminal("[~] Do you want to see chrome in the middle of a brute force operation (yes or no) ? ", "dark_green");
    let chrome = read_input();
    if chrome == "yes" {
        color_terminal("[!] The program will show you chrome and its brute-force operation", "yellow");
        default_config(website, username, path_file, key, true);
    } else {
        color_terminal("[!] You will not see chrome however you will be able to see the logs.", "yellow");
        default_config(website, username, path_file, key, false);
    }
}


//input
pub fn input_email_password_submit() {
    // website for bruteforce
    color_terminal("[~] The site you want to try brute-force : ", "dark_green");
    let website = read_input();

    // username to crack
    color_terminal("[~] Enter the username to brute-force : ", "dark_green");
    let username = read_input();

    // dictionary_password
    color_terminal("[~] Enter the path of the password dictionary : ", "dark_green");
    let path_file = read_input();

    // keyword
    color_terminal("[~] A word to filter passwords (optional): ", "dark_green");
    let key = read_input();

    color_terminal("[~] Do you want to see chrome in the middle of a brute force operation (yes or no) ? ", "dark_green");
    let chrome = read_input();

    if chrome == "yes" {
        color_terminal("[!] The program will show you chrome and its brute-force operation", "yellow");
        config_field_email(website, username, path_file, key, true);
    } else {
        color_terminal("[!] You will not see chrome however you will be able to see the logs.", "yellow");
        config_field_email(website, username, path_file, key, false);
    }
}
pub fn input_text_password_button() {
    // website for bruteforce
    color_terminal("[~] The site you want to try brute-force : ", "dark_green");
    let website = read_input();

    // username to crack
    color_terminal("[~] Enter the username to brute-force : ", "dark_green");
    let username = read_input();

    // dictionary_password
    color_terminal("[~] Enter the path of the password dictionary : ", "dark_green");
    let path_file = read_input();

    // keyword
    color_terminal("[~] A word to filter passwords (optional): ", "dark_green");
    let key = read_input();

    color_terminal("[~] Do you want to see chrome in the middle of a brute force operation (yes or no) ? ", "dark_green");
    let chrome = read_input();
    if chrome == "yes" {
        color_terminal("[!] The program will show you chrome and its brute-force operation", "yellow");
        config_field_button(website, username, path_file, key,true);
    } else {
        color_terminal("[!] You will not see chrome however you will be able to see the logs.", "yellow");
        config_field_button(website, username, path_file, key,false);
    }
}
pub fn input_email_password_button() {
    // website for bruteforce
    color_terminal("[~] The site you want to try brute-force : ", "dark_green");
    let website = read_input();

    // username to crack
    color_terminal("[~] Enter the username to brute-force : ", "dark_green");
    let username = read_input();

    // dictionary_password
    color_terminal("[~] Enter the path of the password dictionary : ", "dark_green");
    let path_file = read_input();

    // keyword
    color_terminal("[~] A word to filter passwords (optional): ", "dark_green");
    let key = read_input();

    color_terminal("[~] Do you want to see chrome in the middle of a brute force operation (yes or no) ? ", "dark_green");
    let chrome = read_input();
    if chrome == "yes" {
        color_terminal("[!] The program will show you chrome and its brute-force operation", "yellow");
        config_field_email_button(website, username, path_file, key, true);
    } else {
        color_terminal("[!] You will not see chrome however you will be able to see the logs.", "yellow");
        config_field_email_button(website, username, path_file, key, false);
    }

}
pub fn input_button() {
    // website for bruteforce
    color_terminal("[~] The site you want to try brute-force : ", "dark_green");
    let website = read_input();

    // input button
    color_terminal("[~] Enter the button (login) selector : ", "dark_green");
    let btn = read_input();

    // username to crack
    color_terminal("[~] Enter the username to brute-force : ", "dark_green");
    let username = read_input();

    // dictionary_password
    color_terminal("[~] Enter the path of the password dictionary : ", "dark_green");
    let path_file = read_input();

    // keyword
    color_terminal("[~] A word to filter passwords (optional): ", "dark_green");
    let key = read_input();

    color_terminal("[~] Do you want to see chrome in the middle of a brute force operation (yes or no) ? ", "dark_green");
    let chrome = read_input();
    if chrome == "yes" {
        color_terminal("[!] The program will show you chrome and its brute-force operation", "yellow");
        config_button(website, username, path_file ,btn, key, true);
    } else {
        color_terminal("[!] You will not see chrome however you will be able to see the logs.", "yellow");
        config_button(website, username, path_file ,btn, key, false);
    }

}
pub fn input_email_button() {
    // website for bruteforce
    color_terminal("[~] The site you want to try brute-force : ", "dark_green");
    let website = read_input();

    // input button
    color_terminal("[~] Enter the button (login) selector : ", "dark_green");
    let btn = read_input();

    // username to crack
    color_terminal("[~] Enter the username to brute-force : ", "dark_green");
    let username = read_input();

    // dictionary_password
    color_terminal("[~] Enter the path of the password dictionary : ", "dark_green");
    let path_file = read_input();

    // keyword
    color_terminal("[~] A word to filter passwords (optional): ", "dark_green");
    let key = read_input();

    color_terminal("[~] Do you want to see chrome in the middle of a brute force operation (yes or no) ? ", "dark_green");
    let chrome = read_input();
    if chrome == "yes" {
        color_terminal("[!] The program will show you chrome and its brute-force operation", "yellow");
        config_email_button(website, username, path_file ,btn, key, true);
    } else {
        color_terminal("[!] You will not see chrome however you will be able to see the logs.", "yellow");
        config_email_button(website, username, path_file ,btn, key, false);
    }

}
pub fn default() {
    // website for bruteforce
    color_terminal("[~] The site you want to try brute-force : ", "dark_green");
    let website = read_input();

    // input username
    color_terminal("[~] Enter the username selector : ", "dark_green");
    let username_input = read_input();

    // input password
    color_terminal("[~] Enter the password selector : ", "dark_green");
    let password_input = read_input();

    // input button
    color_terminal("[~] Enter the button (login) selector : ", "dark_green");
    let form_button = read_input();

    // username to crack
    color_terminal("[~] Enter the username to brute-force : ", "dark_green");
    let username = read_input();

    // dictionary_password
    color_terminal("[~] Enter the path of the password dictionary : ", "dark_green");
    let path_file = read_input();

    // keyword
    color_terminal("[~] A word to filter passwords (optional): ", "dark_green");
    let key = read_input();

    color_terminal("[~] Do you want to see chrome in the middle of a brute force operation (yes or no) ? ", "dark_green");
    let chrome = read_input();

    if chrome == "yes" {
        color_terminal("[!] The program will show you chrome and its brute-force operation", "yellow");
        brute_website(website, username_input, password_input, form_button, username, path_file, key, true);

    } else {
        color_terminal("[!] You will not see chrome however you will be able to see the logs.", "yellow");
        brute_website(website, username_input, password_input, form_button, username, path_file, key, false);

    }


}

//email
pub fn brute_email() {
    color_terminal("[~] Enter the imap address of the service with which you want to brute-force : ", "dark_green");
    let imap = read_input();

    color_terminal("Enter a brute-force email address : ", "dark_green");
    let username = read_input();

    // dictionary_password
    color_terminal("[~] Enter the path of the password dictionary : ", "dark_green");
    let path_file = read_input();

    // keyword
    color_terminal("[~] A word to filter passwords (optional): ", "dark_green");
    let key = read_input();

    brute_mail(username, path_file, key,imap);
}
pub fn brute_email_outlook() {
    color_terminal("[~] Enter the outlook address to brute-force : ", "dark_green");
    let username = read_input();

    // dictionary_password
    color_terminal("[~] Enter the path of the password dictionary : ", "dark_green");
    let path_file = read_input();

    // keyword
    color_terminal("[~] A word to filter passwords (optional): ", "dark_green");
    let key = read_input();

    brute_mail(username, path_file, key,String::from("imap-mail.outlook.com"));
}
pub fn brute_email_icloud() {
    color_terminal("[~] Enter the brute-force icloud address : ", "dark_green");
    let username = read_input();

    // dictionary_password
    color_terminal("[~] Enter the path of the password dictionary : ", "dark_green");
    let path_file = read_input();

    // keyword
    color_terminal("[~] A word to filter passwords (optional): ", "dark_green");
    let key = read_input();

    brute_mail(username, path_file, key,String::from("imap.mail.me.com"));
}
pub fn brute_email_yahoo() {
    color_terminal("[~] Enter the brute-force yahoo address : ", "dark_green");
    let username = read_input();

    // dictionary_password
    color_terminal("[~] Enter the path of the password dictionary : ", "dark_green");
    let path_file = read_input();

    // keyword
    color_terminal("[~] A word to filter passwords (optional): ", "dark_green");
    let key = read_input();

    brute_mail(username, path_file, key, String::from("imap.mail.yahoo.com"));
}

pub fn query() {

    // keyword
    color_terminal("[~] Enter the passwords corresponding to a keyword : ", "dark_green");
    let key = read_input();

    // dictionary_password
    color_terminal("[~] Enter the path of the password dictionary : ", "dark_green");
    let path_file = read_input();

    search_by_keyword(key, path_file);
}