use termion::raw::IntoRawMode;

use crate::ansi;
use crate::MAX_LINE_WIDTH;
use crate::PrompterError;
use std::io::{self, Write};

pub fn prompt(input: &str) -> Result<(), PrompterError> {
    let stdout = io::stdout();
    let mut stdout = stdout
        .lock()
        .into_raw_mode()
        .map_err(|_| PrompterError::IOError)?;

    write!(stdout, "{}>{} ", ansi::SEPARATOR_COLOR, ansi::RESET_COLOR).map_err(|_| PrompterError::IOError)?;

    let mut line = String::new();
    for word in input.split_whitespace() {
        if !line.is_empty() && line.len() + 1 + word.len() > MAX_LINE_WIDTH {
            write!(stdout, "{line}\r\n{}| {}", ansi::SEPARATOR_COLOR, ansi::RESET_COLOR).map_err(|_| PrompterError::IOError)?;
            line.clear();
        }

        if !line.is_empty() {
            line.push(' ');
        }
        line.push_str(word);
    }

    if !line.is_empty() {
        write!(stdout, "{line}\r\n").map_err(|_| PrompterError::IOError)?;
    }

    stdout.flush().map_err(|_| PrompterError::IOError)?;

    drop(stdout);
    Ok(())
}
