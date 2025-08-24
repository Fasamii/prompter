use crate::{
    codes,
    error::{self, PromptError, PromptErrorKind},
    prompt_write,
};
use std::io::Write;
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

pub struct Text;

impl Text {
    pub fn new(ask: &str) -> error::Result<String> {
        let stdin = std::io::stdin();
        let stdout = std::io::stdout();
        let mut stdout = stdout
            .lock()
            .into_raw_mode()
            .map_err(|err| error::PromptError::Io {
                kind: error::PromptErrorKind::StdOut,
                source: err,
            })?;

        prompt_write!(stdout, "{}[{ask}]: ", codes::SAVE_CURSOR_POS)?;
        stdout.flush().map_err(|err| PromptError::Io {
            kind: PromptErrorKind::StdOut,
            source: err,
        })?;

        let mut input = String::new();

        for key in stdin.keys() {
            match key.map_err(|err| PromptError::Io {
                kind: PromptErrorKind::StdIn,
                source: err,
            })? {
                Key::Backspace | Key::Left | Key::ShiftLeft | Key::AltLeft | Key::CtrlLeft => {
                    input.pop();
                    prompt_write!(stdout, "\x08\x20\x08")?;
                    Self::render(&mut stdout, ask, &input)?;
                }

                Key::Char('\n') | Key::Down | Key::ShiftDown | Key::AltDown | Key::CtrlDown => {
                    Self::seal(&mut stdout, ask, Some(&input))?;
                    if input.is_empty() {
                        return Err(PromptError::UserInput(PromptErrorKind::UserEmpty));
                    } else {
                        return Ok(input);
                    }
                }

                Key::Esc => {
                    Self::seal(&mut stdout, ask, None)?;
                    return Err(PromptError::UserInput(PromptErrorKind::UserQuit));
                }

                Key::Char(ch) => {
                    input.push(ch);
                    Self::render(&mut stdout, ask, &input)?;
                }
                _ => todo!("what the fuck are you trying to type here"),
            }
        }
        todo!()
    }

    fn render<W: Write>(stdout: &mut W, ask: &str, input: &str) -> error::Result<()> {
        prompt_write!(stdout, "{}[{}]: {}", codes::RESTORE_CURSOR_POS, ask, input)?;
        stdout.flush().map_err(|err| PromptError::Io {
            kind: PromptErrorKind::StdOut,
            source: err,
        })?;
        Ok(())
    }

    fn seal<W: Write>(stdout: &mut W, ask: &str, input: Option<&str>) -> error::Result<()> {
        prompt_write!(stdout, "{}", codes::RESTORE_CURSOR_POS);
        match input {
            Some(input) if input.is_empty() => {
                prompt_write!(
                    stdout,
                    "[{}]{}:{}",
                    ask,
                    codes::SELECTED_FINAL_COLOR,
                    codes::RESET_COLOR
                )?;
            }
            Some(input) => {
                prompt_write!(
                    stdout,
                    "{}[{}]: {}{}{}",
                    codes::RESTORE_CURSOR_POS,
                    ask,
                    codes::SELECTED_FINAL_COLOR,
                    input,
                    codes::RESET_COLOR
                )?;
            }
            None => {
                prompt_write!(
                    stdout,
                    "{}[{}{}{}]{}:{}",
                    codes::RESTORE_CURSOR_POS,
                    codes::FAIL_COLOR,
                    ask,
                    codes::RESET_COLOR,
                    codes::FAIL_COLOR,
                    codes::RESET_COLOR
                )?;
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

pub struct TextDispatch;

impl TextDispatch {}
