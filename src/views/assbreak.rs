use super::color_console::*;

///
/// The assbreak banner.
/// Examples :
/// ```
/// assbreak::new();
/// ```
///
pub struct assbreak;

///
/// A variable allowing to store the assbreak banner.
/// 
pub static ASSBREAK_STR: &'static str = "
:::'###:::::'######:::'######::'########::'########::'########::::'###::::'##:::'##:
::'## ##:::'##... ##:'##... ##: ##.... ##: ##.... ##: ##.....::::'## ##::: ##::'##::
:'##:. ##:: ##:::..:: ##:::..:: ##:::: ##: ##:::: ##: ##::::::::'##:. ##:: ##:'##:::
'##:::. ##:. ######::. ######:: ########:: ########:: ######:::'##:::. ##: #####::::
:#########::..... ##::..... ##: ##.... ##: ##.. ##::: ##...:::: #########: ##. ##:::
:##.... ##:'##::: ##:'##::: ##: ##:::: ##: ##::. ##:: ##::::::: ##.... ##: ##:. ##::
:##:::: ##:. ######::. ######:: ########:: ##:::. ##: ########: ##:::: ##: ##::. ##:";

impl assbreak {


    ///
    /// The assbreak banner.
    /// Examples :
    /// ```
    /// assbreak::new();
    /// ```
    ///
    pub fn new() {
        ColorConsole::new(ASSBREAK_STR.to_string(), Color::Green)
            .colorize_str();
    }
}