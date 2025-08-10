#![allow(clippy::cargo)]
#![allow(unused)]

const MAX_LINE_WIDTH: usize = 50;

pub mod ansi;
pub mod prompt;
pub mod prompt_options;

use std::io::{self, Write, stdout};

#[derive(Debug)]
pub enum PrompterError {
    NoOptionsProvided,
    IOError,
    UserEmpty, 
    UserQuit,
}
