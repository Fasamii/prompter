use crate::{
    codes,
    error::{self, PromptError, PromptErrorKind},
    prompt_write,
};
use std::io::Write;
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

pub struct SliderPos;

impl SliderPos {
    pub fn new(lenght: usize) -> error::Result<usize> {
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
        let mut pos = 0;
        Self::render(&mut stdout, lenght, pos)?;

        for key in stdin.keys() {
            match key.map_err(|err| PromptError::Io {
                kind: error::PromptErrorKind::StdIn,
                source: err,
            })? {
                Key::Char('q' | 'e') | Key::Esc => {
                    Self::seal(&mut stdout, lenght, None)?;
                    prompt_write!(stdout, "\r\n")?;
                    return Err(PromptError::UserInput(error::PromptErrorKind::UserQuit));
                }
                Key::Char('a' | 'h') | Key::Left => {
                    pos = if pos == 0 { lenght - 1 } else { pos - 1 };
                    Self::render(&mut stdout, lenght, pos)?;
                }
                Key::Char('d' | 'l') | Key::Right => {
                    pos = if pos + 1 == lenght { 0 } else { pos + 1 };
                    Self::render(&mut stdout, lenght, pos)?;
                }
                Key::Char('\n' | 's' | 'j') | Key::Down => {
                    Self::seal(&mut stdout, lenght, Some(pos))?;
                    prompt_write!(stdout, "\r\n")?;
                    return Ok(pos);
                }
                _ => (),
            }
        }
        todo!("implement that");
    }
    fn render<W: Write>(stdout: &mut W, lenght: usize, pos: usize) -> error::Result<()> {
        prompt_write!(stdout, "{}", codes::RESTORE_CURSOR_POS)?;
        prompt_write!(
            stdout,
            "{}{}{}{}{}{}{}{}{}",
            codes::SEPARATOR_COLOR,
            codes::SLIDER_SEPARATOR_CHAR.to_string().repeat(pos),
            codes::RESET_COLOR,
            codes::HIGHLIGHT_COLOR,
            codes::SLIDER_SELECTION_CHAR,
            codes::RESET_COLOR,
            codes::SEPARATOR_COLOR,
            codes::SLIDER_SEPARATOR_CHAR
                .to_string()
                .repeat((lenght - 1) - pos),
            codes::RESET_COLOR
        )?;
        stdout.flush().map_err(|err| PromptError::Io {
            kind: PromptErrorKind::StdOut,
            source: err,
        })?;
        Ok(())
    }
    fn seal<W: Write>(stdout: &mut W, lenght: usize, pos: Option<usize>) -> error::Result<()> {
        prompt_write!(stdout, "{}", codes::RESTORE_CURSOR_POS)?;

        match pos {
            Some(pos) => prompt_write!(
                stdout,
                "{}{}{}{}{}{}{}{}{}",
                codes::SEPARATOR_COLOR,
                codes::SLIDER_SEPARATOR_CHAR.to_string().repeat(pos),
                codes::RESET_COLOR,
                codes::SELECTED_FINAL_COLOR,
                codes::SLIDER_SELECTION_CHAR,
                codes::RESET_COLOR,
                codes::SEPARATOR_COLOR,
                codes::SLIDER_SEPARATOR_CHAR
                    .to_string()
                    .repeat((lenght - 1) - pos),
                codes::RESET_COLOR,
            )?,
            None => prompt_write!(
                stdout,
                "{}{}{}",
                codes::FAIL_COLOR,
                codes::SLIDER_SEPARATOR_CHAR.to_string().repeat(lenght + 1),
                codes::RESET_COLOR
            )?,
        }

        stdout.flush().map_err(|err| PromptError::Io {
            kind: PromptErrorKind::StdOut,
            source: err,
        })?;

        Ok(())
    }
}

pub struct SliderFill;

impl SliderFill {
    pub fn new(lenght: usize) -> error::Result<usize> {
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
        let mut pos = 0;
        Self::render(&mut stdout, lenght, pos)?;

        for key in stdin.keys() {
            match key.map_err(|err| PromptError::Io {
                kind: error::PromptErrorKind::StdIn,
                source: err,
            })? {
                Key::Char('q' | 'e') | Key::Esc => {
                    Self::seal(&mut stdout, lenght, None)?;
                    prompt_write!(stdout, "\r\n")?;
                    return Err(PromptError::UserInput(error::PromptErrorKind::UserQuit));
                }
                Key::Char('a' | 'h') | Key::Left => {
                    pos = if pos == 0 { lenght - 1 } else { pos - 1 };
                    Self::render(&mut stdout, lenght, pos)?;
                }
                Key::Char('d' | 'l') | Key::Right => {
                    pos = if pos + 1 == lenght { 0 } else { pos + 1 };
                    Self::render(&mut stdout, lenght, pos)?;
                }
                Key::Char('\n' | 's' | 'j') | Key::Down => {
                    Self::seal(&mut stdout, lenght, Some(pos))?;
                    prompt_write!(stdout, "\r\n")?;
                    return Ok(pos);
                }
                _ => (),
            }
        }
        todo!("implement that");
    }
    fn render<W: Write>(stdout: &mut W, lenght: usize, pos: usize) -> error::Result<()> {
        prompt_write!(stdout, "{}", codes::RESTORE_CURSOR_POS)?;
        prompt_write!(
            stdout,
            "{}{}{}{}{}{}",
            codes::HIGHLIGHT_COLOR,
            codes::SLIDER_SELECTION_CHAR.to_string().repeat(pos + 1),
            codes::RESET_COLOR,
            codes::SEPARATOR_COLOR,
            codes::SLIDER_SEPARATOR_CHAR
                .to_string()
                .repeat((lenght - 1) - pos),
            codes::RESET_COLOR
        )?;
        stdout.flush().map_err(|err| PromptError::Io {
            kind: PromptErrorKind::StdOut,
            source: err,
        })?;
        Ok(())
    }
    fn seal<W: Write>(stdout: &mut W, lenght: usize, pos: Option<usize>) -> error::Result<()> {
        prompt_write!(stdout, "{}", codes::RESTORE_CURSOR_POS)?;

        match pos {
            Some(pos) => prompt_write!(
                stdout,
                "{}{}{}{}{}{}",
                codes::SELECTED_FINAL_COLOR,
                codes::SLIDER_SELECTION_CHAR.to_string().repeat(pos + 1),
                codes::RESET_COLOR,
                codes::SEPARATOR_COLOR,
                codes::SLIDER_SEPARATOR_CHAR
                    .to_string()
                    .repeat((lenght - 1) - pos),
                codes::RESET_COLOR,
            )?,
            None => prompt_write!(
                stdout,
                "{}{}{}",
                codes::FAIL_COLOR,
                codes::SLIDER_SEPARATOR_CHAR
                    .to_string()
                    .repeat((lenght - 1) + 1),
                codes::RESET_COLOR
            )?,
        }

        stdout.flush().map_err(|err| PromptError::Io {
            kind: PromptErrorKind::StdOut,
            source: err,
        })?;

        Ok(())
    }
}
