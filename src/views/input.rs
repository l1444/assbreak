use std::io::{Write};
use super::color_console::*;
use super::filter::*;
///
/// A strcuture for declare, the functions available on Input
/// 
pub struct Input;

impl Input {

    ///
    /// Declare a new instance of Input for use a Input
    /// 
    pub fn new(text: &str) -> String {
        loop {
            ColorConsole::new(text.to_string(), Color::DarkGreen).colorize_str();
            let mut v = String::new();
            let _ = std::io::stdout().flush();
            match std::io::stdin().read_line(&mut v) {
                Ok(_) => {
                    if !String::from(&v).is_empty() {
                        break Filter::new(v, true, true).filter_now();
                    } else {
                        continue;
                    }
                }
                Err(_) => {
                    ColorConsole::new("A error has occured :/".to_string(), Color::Red);
                    continue;
                }
            }
        }
    }

}