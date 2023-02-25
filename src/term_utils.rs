pub const ESC: &str = "\x1b";
pub const RESET: &str = "\x1b[0m";
pub const LINE_CLEAR: &str = "\x1b[2K";
pub const SCREEN_CLEAR: &str = "\x1b[2J\x1b[H";
pub const BACK_ONE_LINE: &str = "\x1b[1F";
pub const WIDTH: usize = 52;

/// Format foreground color escape code.
pub fn fg_color(r: u8, g: u8, b: u8) -> String {
    format!("{ESC}[38;2;{r};{g};{b}m")
}

/// Terminal bell
pub fn alert() {
    print!("\x07");
}
