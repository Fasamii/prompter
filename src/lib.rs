#![allow(clippy::cargo)]
#![allow(unused)]

const MAX_LINE_WIDTH: usize = 50;

const SAVE_CURSOR: &'static str = "\x1b[s";
const RESTORE_CURSOR: &'static str = "\x1b[u";

pub mod ansi;
pub mod prompt;
pub mod prompt_options;

pub use prompt::prompt;
pub use prompt_options::PromptOptions;

use std::io::{self, Write, stdout};

#[derive(Debug)]
pub enum PrompterError {
    NoOptionsProvided,
    IOError,
    UserEmpty, 
    UserQuit,
}
