use crate::{PrompterError, ansi};
use std::io::{self, Write};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

pub fn prompt_options_multiple(
    items: Vec<Vec<&str>>,
) -> Result<Vec<Result<usize, PrompterError>>, PrompterError> {
    let mut ret: Vec<Result<usize, PrompterError>> = Vec::with_capacity(items.len());

    for prompt in items {
        ret.push(PromptOptions::new(&prompt));
    }

    Ok(ret)
}

pub struct PromptOptions;

impl PromptOptions {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(items: &Vec<&str>) -> Result<usize, PrompterError> {
        if items.is_empty() {
            return Err(PrompterError::NoOptionsProvided);
        }

        let stdout = io::stdout();
        let mut stdout = stdout
            .lock()
            .into_raw_mode()
            .map_err(|_| PrompterError::IOError)?;
        let stdin = io::stdin();

        let mut selected = 0;
        Self::render(&mut stdout, items, selected)?;

        for key in stdin.keys() {
            match key.map_err(|_| PrompterError::IOError)? {
                Key::Char('q' | 'e') | Key::Esc => {
                    Self::seal(&mut stdout, items, None)?;
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
                    Self::seal(&mut stdout, items, Some(selected))?;
                    write!(stdout, "\r\n").map_err(|_| PrompterError::IOError)?;
                    return Ok(selected);
                }
                _ => {}
            }
        }
        Err(PrompterError::IOError)
    }

    fn render<W: Write>(
        stdout: &mut W,
        items: &[&str],
        selected: usize,
    ) -> Result<(), PrompterError> {
        write!(stdout, "\r").map_err(|_| PrompterError::IOError)?;

        for (idx, item) in items.iter().enumerate() {
            if idx == selected {
                write!(
                    stdout,
                    "{}{}{}",
                    ansi::HIGHLIGHT_COLOR,
                    item,
                    ansi::RESET_COLOR
                )
                .map_err(|_| PrompterError::IOError)?;
            } else {
                write!(
                    stdout,
                    "{}{}{}",
                    ansi::NORMAL_COLOR,
                    item,
                    ansi::RESET_COLOR
                )
                .map_err(|_| PrompterError::IOError)?;
            }

            if idx < items.len() - 1 {
                write!(stdout, "{} :: {}", ansi::SEPARATOR_COLOR, ansi::RESET_COLOR)
                    .map_err(|_| PrompterError::IOError)?;
            }
            stdout.flush().map_err(|_| PrompterError::IOError)?;
        }
        Ok(())
    }

    fn seal<W: Write>(
        stdout: &mut W,
        items: &[&str],
        selected: Option<usize>,
    ) -> Result<(), PrompterError> {
        write!(stdout, "\r").map_err(|_| PrompterError::IOError)?;

        for (idx, item) in items.iter().enumerate() {
            let color = match selected {
                None if items.len() == 1 => ansi::FAIL_COLOR,
                None => ansi::SEPARATOR_COLOR,
                Some(sel_idx) if idx == sel_idx => ansi::SELECTED_COLOR,
                Some(_) => ansi::SEPARATOR_COLOR,
            };

            write!(stdout, "{}{}{}", color, item, ansi::RESET_COLOR)
                .map_err(|_| PrompterError::IOError)?;

            if idx + 1 < items.len() {
                let separator_color = match selected {
                    None => ansi::FAIL_COLOR,
                    Some(_) => ansi::SEPARATOR_COLOR,
                };
                write!(stdout, "{} :: {}", separator_color, ansi::RESET_COLOR)
                    .map_err(|_| PrompterError::IOError)?;
            }
        }
        stdout.flush().map_err(|_| PrompterError::IOError)?;
        Ok(())
    }
}
