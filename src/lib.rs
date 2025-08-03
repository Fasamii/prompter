#![allow(clippy::cargo)]
#![allow(unused)]
use std::io::{self, Write, stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

const MAX_LINE_WIDTH: usize = 50;

mod ansi {
    pub const NORMAL_COLOR: &str = "\x1B[37m";
    pub const SEPARATOR_COLOR: &str = "\x1B[90m";
    pub const HIGHLIGHT_COLOR: &str = "\x1B[47;30m";
    pub const SELECTED_COLOR: &str = "\x1B[32;5;1m";
    pub const FAIL_SEPARATOR_COLOR: &str = "\x1B[38;5;1m";
    pub const RESET_COLOR: &str = "\x1B[0m";
}

#[derive(Debug)]
pub enum PrompterError {
    NoOptionsProvided,
    UserQuit,
    IOError,
}

pub fn write_prompt<W: Write>(out: &mut W, input: &str) -> Result<(), PrompterError> {
    write!(out, "> ").map_err(|_| PrompterError::IOError)?;

    let mut line = String::new();
    for word in input.split_whitespace() {
        if !line.is_empty() && line.len() + 1 + word.len() > MAX_LINE_WIDTH {
            write!(out, "{line}\r\n").map_err(|_| PrompterError::IOError)?;
            line.clear();
        }

        if !line.is_empty() {
            line.push(' ');
        }
        line.push_str(word);
    }

    if !line.is_empty() {
        write!(out, "{line}\r\n").map_err(|_| PrompterError::IOError)?;
    }

    out.flush().map_err(|_| PrompterError::IOError)?;
    Ok(())
}

pub struct WritePromptOptions;

impl WritePromptOptions {
    const SAVE_CURSOR: &'static str = "\x1B[s";
    const RESTORE_CURSOR: &'static str = "\x1B[u";

    #[allow(clippy::new_ret_no_self)]
    pub fn new(items: &Vec<&str>) -> Result<usize, PrompterError> {
        if items.is_empty() {
            return Err(PrompterError::NoOptionsProvided);
        }

        let stdout = stdout();
        let mut stdout = stdout
            .lock()
            .into_raw_mode()
            .map_err(|_| PrompterError::IOError)?;
        let stdin = io::stdin();

        write!(stdout, "{}", Self::SAVE_CURSOR).map_err(|_| PrompterError::IOError)?;

        let mut selected = 0;
        Self::render(&mut stdout, items, selected)?;

        for key in stdin.keys() {
            match key.map_err(|_| PrompterError::IOError)? {
                Key::Char('q' | 'e') | Key::Esc => {
                    Self::render_final(&mut stdout, items, None)?;
                    write!(stdout, "\r\n").map_err(|_| PrompterError::IOError)?;
                    return Err(PrompterError::UserQuit);
                }
                Key::Char('a' | 'h') | Key::Left => {
                    selected = if selected == 0 {
                        items.len() - 1
                    } else {
                        selected - 1
                    };
                    Self::render(&mut stdout, items, selected)?;
                }
                Key::Char('d' | 'l') | Key::Right => {
                    selected = if selected + 1 >= items.len() {
                        0
                    } else {
                        selected + 1
                    };
                    Self::render(&mut stdout, items, selected)?;
                }
                Key::Char('\n' | 'j' | 's') => {
                    Self::render_final(&mut stdout, items, Some(selected))?;
                    write!(stdout, "\r\n").map_err(|_| PrompterError::IOError)?;
                    return Ok(selected);
                }
                _ => {}
            }
        }

        Err(PrompterError::IOError)
    }

    fn render<W: Write>(out: &mut W, items: &[&str], selected: usize) -> Result<(), PrompterError> {
        write!(out, "{}", Self::RESTORE_CURSOR).map_err(|_| PrompterError::IOError)?;

        for (i, item) in items.iter().enumerate() {
            if i == selected {
                write!(
                    out,
                    "{}{}{}",
                    ansi::HIGHLIGHT_COLOR,
                    item,
                    ansi::RESET_COLOR
                )
                .map_err(|_| PrompterError::IOError)?;
            } else {
                write!(out, "{}{}{}", ansi::NORMAL_COLOR, item, ansi::RESET_COLOR)
                    .map_err(|_| PrompterError::IOError)?;
            }

            if i < items.len() - 1 {
                write!(out, "{} :: {}", ansi::SEPARATOR_COLOR, ansi::RESET_COLOR)
                    .map_err(|_| PrompterError::IOError)?;
            }
        }

        out.flush().map_err(|_| PrompterError::IOError)?;
        Ok(())
    }

    fn render_final<W: Write>(
        out: &mut W,
        items: &[&str],
        selected: Option<usize>,
    ) -> Result<(), PrompterError> {
        write!(out, "{}", Self::RESTORE_CURSOR).map_err(|_| PrompterError::IOError)?;
        for (i, item) in items.iter().enumerate() {
            let color = match selected {
                None if items.len() == 1 => ansi::FAIL_SEPARATOR_COLOR,
                None => ansi::NORMAL_COLOR,
                Some(sel_idx) if i == sel_idx => ansi::SELECTED_COLOR,
                Some(_) => ansi::SEPARATOR_COLOR,
            };

            write!(out, "{}{}{}", color, item, ansi::RESET_COLOR)
                .map_err(|_| PrompterError::IOError)?;

            if i + 1 < items.len() {
                let separator_color = match selected {
                    None => ansi::FAIL_SEPARATOR_COLOR,
                    Some(_) => ansi::SEPARATOR_COLOR,
                };
                write!(out, "{} :: {}", separator_color, ansi::RESET_COLOR)
                    .map_err(|_| PrompterError::IOError)?;
            }
        }
        out.flush().map_err(|_| PrompterError::IOError)?;
        Ok(())
    }
}

pub fn prompt(input: &str, opts: &Vec<&str>) -> Result<usize, PrompterError> {
    let stdout = stdout();
    let mut stdout = stdout
        .lock()
        .into_raw_mode()
        .map_err(|_| PrompterError::IOError)?;

    write_prompt(&mut stdout, input)?;
    drop(stdout);
    WritePromptOptions::new(opts)
}

pub fn prompt_multiple(
    input: &str,
    opts: Vec<Vec<&str>>,
) -> Result<Vec<Result<usize, PrompterError>>, PrompterError> {
    let stdout = stdout();
    let mut stdout = stdout
        .lock()
        .into_raw_mode()
        .map_err(|_| PrompterError::IOError)?;

    write_prompt(&mut stdout, input)?;
    drop(stdout);

    let mut ret: Vec<Result<usize, PrompterError>> = Vec::with_capacity(opts.len());

    for selection in opts {
        ret.push(WritePromptOptions::new(&selection));
    }

    Ok(ret)
}
