pub mod options;
pub mod slider;
pub mod text;

#[macro_export]
macro_rules! prompt_write {
    ($dst:expr, $($arg:tt)*) => {
        write!($dst, $($arg)*).map_err(|err| PromptError::Io {
            kind: error::PromptErrorKind::StdOut,
            source: err,
        })
    };
}
