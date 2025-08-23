mod codes {
    pub const SAVE_CURSOR_POS: &'static str = "\x1b[s";
    pub const RESTORE_CURSOR_POS: &'static str = "\x1b[u";

    pub const SLIDER_SEPARATOR_CHAR: char = ':';
    pub const SLIDER_SELECTION_CHAR: char = ';';

    pub const NORMAL_COLOR: &'static str = "\x1b[37m";
    pub const SEPARATOR_COLOR: &'static str = "\x1b[90m";
    pub const HIGHLIGHT_COLOR: &'static str = "\x1b[47;30m";
    pub const SELECTED_FINAL_COLOR: &'static str = "\x1b[32;5;1m";

    pub const FAIL_COLOR: &'static str = "\x1b[38;5;1m";

    pub const RESET_COLOR: &'static str = "\x1b[0m";
}

pub mod error;
pub mod prompt;
