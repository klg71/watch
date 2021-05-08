pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

pub const TERMINAL_COLOR_ORANGE: &str = "\x1B[33m";
pub const TERMINAL_COLOR_RESET: &str = "\x1B[0m";
