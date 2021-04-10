use std::process::exit;
use console::Style;

use crate::{
    views::{
        color_console::{
            ColorConsole, 
            Color
        }
    }
};
use crate::views::count_all_characters;

#[derive(Debug)]
pub enum ASSBREAK_ERROR{
    // une erreur inéxpliqué 
    Err0,
    // ayant inséré un mauvais type de caractère
    Err1,
    // une option non existante
    Err2,
    // une erreur dès l'entrée de quelque chose dans un champ de texte (io::stdin().read_line())
    Err3,
    // problème de conversion
    Err4,
    // impossible de demarrer une action sans connexion
    Err5,
    // Vous n'avez rien rentré
    Err6
}

impl ASSBREAK_ERROR {
    pub fn new(err: ASSBREAK_ERROR) {
        match err {
            ASSBREAK_ERROR::Err0 => ASSBREAK_ERROR::display_error("[Err0] An unexplainable error has just occurred.", true),
            ASSBREAK_ERROR::Err1 => ASSBREAK_ERROR::display_error("[Err1] You have inserted the wrong type of character.", true),
            ASSBREAK_ERROR::Err2 => ASSBREAK_ERROR::display_error("[Err2] This option does not exist.", false),
            ASSBREAK_ERROR::Err3 => ASSBREAK_ERROR::display_error("[Err3] This option does not exist.", false), 
            ASSBREAK_ERROR::Err4 => ASSBREAK_ERROR::display_error("[Err4] Conversion problem between 2 types.", false),
            ASSBREAK_ERROR::Err5 => ASSBREAK_ERROR::display_error("[Err5] You do not have an internet connection, therefore you are not allowed to run this tool.", false),
            ASSBREAK_ERROR::Err6 => ASSBREAK_ERROR::display_error("[Err6] You must enter something in this text field.", false),
            _ => {}
        }
    }

    fn display_error(err_str: &str, quit: bool) {
        ColorConsole::colorized("\nAn error has occurred : \n \n ".to_owned(), Color::Red);
        ColorConsole::colorized(format!(" {}", err_str), Color::BgRed);
        if quit == true {
            ColorConsole::colorized("\n See you next time :)".to_string(), Color::Red);
            exit(1);
        }
    }
}
