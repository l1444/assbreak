///
/// displays a large text written assbreak (the banner what)
///
pub mod assbreak;
///
/// allows you to change the color of a text on the terminal.
///
pub mod color_console;
///
/// displays alerts for different situations.
///
pub mod alert;
///
/// filter the spaces and enter them.
///
pub mod filter;
///
/// a field for writing text.
///
pub mod input;

use unicode_segmentation::UnicodeSegmentation;

pub fn count_all_characters(str: &str) -> usize {
    return str.graphemes(true).count()
}