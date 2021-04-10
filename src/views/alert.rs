use super::color_console::*;
use std::format;

///
/// Create a new alert.
/// Examples :
/// ```
/// let x = Alert::new("warning")
///         .alert_website("password", "admins", 342);
/// ```
///
pub struct Alert {
    text: String,
    result: String,
}

///
/// Just an intermediary, to be able to make the code work normally.
/// 
struct AlertMessage {
    str: String
}

impl Alert {

    ///
    /// Create a new alert.
    /// Examples :
    /// ```
    /// let x = Alert::new("warning");
    ///
    /// ```
    ///
    pub fn new(result: &str) -> Self {
        Alert {
            text: String::new(),
            result: String::from(result),
        }
    }

    ///
    /// Make an alert with a demo for a brute-force website.
    /// Examples :
    /// ```
    /// let x = Alert::new("warning")
    ///         .alert_website("password", "admins", 342);
    /// ```
    ///
    pub fn alert_website(&self, pass: String, username: String, attempt: u128) {
        let am = AlertMessage::new(format!("[PASSWORD] => {}, [USERNAME] => {}, ATTEMPT => {}", pass, username, attempt));
        Alert::alert_type(&self, am);
    }

    ///
    /// Make an alert with a demo for a brute-force email.
    /// Examples :
    /// ```
    /// let x = Alert::new("warning")
    ///         .alert_website("password", "contact@is.fr", 667);
    /// ```
    ///
    pub fn alert_email(&self, pass: String, email: String, attempt: u128) {
        let am = AlertMessage::new(format!("[PASSWORD] => {}, [USERNAME] => {}, ATTEMPT => {}", pass, email, attempt));
        Alert::alert_type(&self, am);
    }

    ///
    /// Here, there are 4 types of alert.
    ///
    fn alert_type(&self, am: AlertMessage) {
        match String::as_str(&self.result) {
            "success" => {
                ColorConsole::new(format!("[!] [TRIED] => SUCCESS | {}", am.str), Color::Green)
                    .colorize_str();
            }
            "false" => {
                ColorConsole::new(format!("[!] [TRIED] => FALSE | {}", am.str), Color::Red)
                    .colorize_str();
            }
            "warning" => {
                ColorConsole::new(format!("[!] [WARNING] | {}", am.str), Color::Yellow)
                    .colorize_str();
            }
            "error" => {
                ColorConsole::new(format!("[x] [ERROR] | {}", am.str), Color::Red)
                    .colorize_str();
            }
            _ => println!("{}", am.str)
        }
    }
}

///
/// Examples : 
/// ```
/// let alert_message = AlertMessage::new("string");
/// // to string
/// let string = alert_message.to_string();
/// 
impl AlertMessage {

    ///
    /// Create a new AlertMessage
    /// 
    pub fn new(str: String) -> Self {
        AlertMessage {
            str
        }
    }
    ///
    /// Convert AlertMessage to &String :)
    /// 
    pub fn to_string(&self) -> &String {
        return &self.str
    }
}