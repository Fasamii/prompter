use crate::{
    codes,
    error::{self, PromptError},
    prompt_write,
};
use std::io::Write;
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

pub struct Options;

impl Options {
    pub fn new(options: &Vec<&str>) -> crate::error::Result<usize> {
        let stdin = std::io::stdin();
        let stdout = std::io::stdout();
        let mut stdout = stdout
            .lock()
            .into_raw_mode()
            .map_err(|err| error::PromptError::Io {
                kind: error::PromptErrorKind::StdOut,
                source: err,
            })?;

        prompt_write!(stdout, "{}", codes::SAVE_CURSOR_POS)?;

        let mut selected = 0;
        Self::render(&mut stdout, &options, selected)?;

        for key in stdin.keys() {
            match key.map_err(|err| PromptError::Io {
                kind: error::PromptErrorKind::StdIn,
                source: err,
            })? {
                Key::Char('q' | 'e') | Key::Esc => {
                    Self::seal(&mut stdout, &options, None)?;
                    return Err(PromptError::UserInput(error::PromptErrorKind::UserQuit));
                }
                Key::Char('a' | 'h') | Key::Left => {
                    selected = if selected == 0 {
                        options.len() - 1
                    } else {
                        selected - 1
                    };
                    Self::render(&mut stdout, &options, selected)?;
                }
                Key::Char('d' | 'l') | Key::Right => {
                    selected = if selected + 1 == options.len() {
                        0
                    } else {
                        selected + 1
                    };
                    Self::render(&mut stdout, &options, selected)?;
                }
                Key::Char('\n' | 's' | 'j') | Key::Down => {
                    Self::seal(&mut stdout, &options, Some(selected))?;
                    return Ok(selected);
                }
                _ => {}
            }
        }
        Ok(12)
    }

    fn render<W: Write>(stdout: &mut W, options: &[&str], selected: usize) -> error::Result<()> {
        prompt_write!(stdout, "{}", codes::RESTORE_CURSOR_POS)?;

        for (idx, option) in options.iter().enumerate() {
            if idx == selected {
                prompt_write!(
                    stdout,
                    "{}{}{}",
                    codes::HIGHLIGHT_COLOR,
                    option,
                    codes::RESET_COLOR
                )?;
            } else {
                prompt_write!(
                    stdout,
                    "{}{}{}",
                    codes::NORMAL_COLOR,
                    option,
                    codes::RESET_COLOR
                )?;
            }

            if idx + 1 < options.len() {
                prompt_write!(
                    stdout,
                    " {}::{} ",
                    codes::SEPARATOR_COLOR,
                    codes::RESET_COLOR
                )?;
            }
        }
        stdout.flush().map_err(|err| PromptError::Io {
            kind: error::PromptErrorKind::StdOut,
            source: err,
        })?;

        Ok(())
    }

    fn seal<W: Write>(
        stdout: &mut W,
        options: &[&str],
        selected: Option<usize>,
    ) -> error::Result<()> {
        prompt_write!(stdout, "{}", codes::RESTORE_CURSOR_POS)?;

        for (idx, &option) in options.iter().enumerate() {
            let color = match selected {
                None if options.len() == 1 => codes::FAIL_COLOR,
                None => codes::SEPARATOR_COLOR,
                Some(selected) if idx == selected => codes::SELECTED_FINAL_COLOR,
                Some(_) => codes::SEPARATOR_COLOR,
            };

            prompt_write!(stdout, "{}{}{}", color, option, codes::RESET_COLOR)?;

            if idx + 1 < options.len() {
                let color = match selected {
                    None => codes::FAIL_COLOR,
                    Some(_) => codes::SEPARATOR_COLOR,
                };

                prompt_write!(stdout, " {}::{} ", color, codes::RESET_COLOR)?;
            }
        }

        prompt_write!(stdout, "\r\n")?;
        stdout.flush().map_err(|err| PromptError::Io {
            kind: error::PromptErrorKind::StdOut,
            source: err,
        })?;

        Ok(())
    }
}
