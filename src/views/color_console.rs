use console::Style;
use super::count_all_characters;
///
/// An enumeration that allows you to store all the fundamental colors to use assbreak!
/// 
pub enum Color {
    Blue,
    Red,
    Green,
    DarkGreen, 
    Yellow,
    Purple,
    BgRed,
    BgGreen,
    BgYellow
}

///
/// The structure to be able to use the functions in the ColorConsole
/// Examples :
/// ```
/// ColorConsole::new("lorem ipsum", "purple");
///
pub struct ColorConsole {
    text: String,
    color: Color,
}


impl ColorConsole {

    ///
    /// Initialize the functions of ColorConsole.
    /// Examples :
    /// ```
    /// ColorConsole::new("lorem ipsum", "purple");
    ///
    pub fn new(text: String, color: Color) -> Self{
        ColorConsole {
            text,
            color
       }
    }

    pub fn colorized(text: String, color: Color) -> () {
        let color = ColorConsole {
            text,
            color
       };
       color.colorize_str();
    }
    ///
    /// This function colors a print.
    /// Examples :
    /// ```
    /// ColorConsole::new("lorem ipsum", "purple")
    ///             .colorize_str();
    ///
    pub fn colorize_str(self) {
        match self.color {
            Color::Blue => {
                #[cfg(not(target_os = "windows"))]
                let blue = Style::new().bright().blue();
                println!("{}", blue.apply_to(&self.text));
                #[cfg(target_os = "windows")]
                println!("{}", &self.text);
            },
            Color::Red => {
                #[cfg(not(target_os = "windows"))]
                let red = Style::new().bright().red();
                println!("{}", red.apply_to(&self.text));
                #[cfg(target_os = "windows")]
                println!("{}", &self.text);
            },
            Color::Green =>  {
                #[cfg(not(target_os = "windows"))]
                let green = Style::new().bright().green();
                println!("{}", green.apply_to(&self.text));
                #[cfg(target_os = "windows")]
                println!("{}", &self.text);
            },
            // for input::new()
            Color::DarkGreen =>  {
                #[cfg(not(target_os = "windows"))]
                let green = Style::new().green();
                print!("{}", green.apply_to(&self.text));
                #[cfg(target_os = "windows")]
                print!("{}", &self.text);
            },
            Color::Yellow => {
                #[cfg(not(target_os = "windows"))]
                let yellow = Style::new().bright().yellow();
                println!("{}", yellow.apply_to(&self.text));
                #[cfg(target_os = "windows")]
                println!("{}", &self.text);
            },
            Color::BgRed => {
                #[cfg(not(target_os = "windows"))]
                let bg_red = Style::new().bg(console::Color::Red);
                println!("\n  {}  \n", bg_red.apply_to(&self.text));
                #[cfg(target_os = "windows")]
                println!("{}", &self.text);
            }
            Color::BgGreen => {
                #[cfg(not(target_os = "windows"))]
                let bg_green = Style::new().bg(console::Color::Green);
                println!("\n  {}  \n", bg_green.apply_to(&self.text));
                #[cfg(target_os = "windows")]
                println!("{}", &self.text);
            }
            Color::BgYellow => {
                #[cfg(not(target_os = "windows"))]
                let bg_yell = Style::new().bg(console::Color::Yellow);
                println!("\n  {}  \n", bg_yell.apply_to(&self.text));
                #[cfg(target_os = "windows")]
                println!("{}", &self.text);
            }
            _ => {
                println!("{}", &self.text);
            }
        }
    }
}