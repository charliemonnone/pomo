pub const ESC: &str = "\x1b";
pub const RESET: &str = "\x1b[0m";
pub const LINE_CLEAR: &str = "\x1b[2K";
pub const SCREEN_CLEAR: &str = "\x1b[2J\x1b[H";
pub const BACK_ONE_LINE: &str = "\x1b[1F";

pub const BOX_CHAR: &str = "\u{2500}";
pub const VERTICAL_BAR: &str = "\u{2502}";
pub const WIDTH: usize = 52;

pub const BOX_CHARS: &'static [&'static str] = &["╭", "╮", "╯", "╰", "─", "│"];

pub fn bg_color(r: u8, g: u8, b: u8) -> String {
    format!("{ESC}[48;2;{r};{g};{b}m")
}

pub fn fg_color(r: u8, g: u8, b: u8) -> String {
    format!("{ESC}[38;2;{r};{g};{b}m")
}
