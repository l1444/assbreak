// local
use super::brute::brute_force;

///
/// default_config => field username : input[type=text] && field password : input[type=password] && button : input[type=button]
///
pub fn default_config(addr: String, username: String, dictionary: String, key: String, show_chrome: bool) {
    brute_force(addr, String::from("input[type=text]"), String::from("input[type=password]"), String::from("input[type=submit]"), username, dictionary, key, show_chrome);
}

///
/// config_field_email => field username : input[type=email] && field password : input[type=password] && button : input[type=button]
///
pub fn config_field_email(addr: String, username: String, dictionary: String, key: String,show_chrome: bool) {
    brute_force(addr, String::from("input[type=email]"), String::from("input[type=password]"), String::from("input[type=submit]"), username, dictionary, key, show_chrome);
}

///
/// config_field_button => field username : input[type=text] && field password : input[type=password] && button : <button>
///
pub fn config_field_button(addr: String, username: String, dictionary: String, key: String,show_chrome: bool) {
    brute_force(addr, String::from("input[type=text]"), String::from("input[type=password]"), String::from("button"), username, dictionary, key, show_chrome);
}


///
/// config_field_email_button => field username : input[type=email] && field password : input[type=password] && button : <button>
///
pub fn config_field_email_button(addr: String, username: String, dictionary: String, key: String,show_chrome: bool) {
    brute_force(addr, String::from("input[type=email]"), String::from("input[type=password]"), String::from("button"), username, dictionary, key, show_chrome);
}

///
/// config_button => field username : input[type=text] && field password : input[type=password] && button : button
///
pub fn config_button(addr: String, username: String, dictionary: String, key: String, button: String,show_chrome: bool) {
    brute_force(addr, String::from("input[type=text]"), String::from("input[type=password]"), button, username, dictionary, key, show_chrome);
}

///
/// config_email_button => field username : input[type=email] && field password : input[type=password] && button : button
///
pub fn config_email_button(addr: String, username: String, dictionary: String, key: String, button: String,show_chrome: bool) {
    brute_force(addr, String::from("input[type=email]"), String::from("input[type=password]"), button, username, dictionary, key, show_chrome);
}