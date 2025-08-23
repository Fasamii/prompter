#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum PromptErrorKind {
    PassedNoOptions,
    UserQuit,
    UserEmpty,
    StdOut,
    StdIn,
}

#[derive(Debug)]
pub enum PromptError {
    Io {
        kind: PromptErrorKind,
        source: std::io::Error,
    },
    UserInput(PromptErrorKind),
    Creation(PromptErrorKind),
}

impl std::fmt::Display for PromptError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PromptError::Io { kind, source: _ } => write!(f, "std input error: {kind:?}"),
            PromptError::UserInput(kind) => write!(f, "user input error: {kind:?}"),
            PromptError::Creation(kind) => write!(f, "creation of prompt failed: {kind:?}"),
        }
    }
}

impl std::error::Error for PromptError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            PromptError::Io { kind: _, source } => Some(source),
            PromptError::Creation(_) => None,
            PromptError::UserInput(_) => None,
        }
    }
}

impl PromptError {
    pub fn kind(&self) -> PromptErrorKind {
        match self {
            PromptError::Io { kind, source: _ } => *kind,
            PromptError::UserInput(kind) => *kind,
            PromptError::Creation(kind) => *kind,
        }
    }

    pub fn from_stdin(source: std::io::Error) -> Self {
        let kind = match source.kind() {
            std::io::ErrorKind::UnexpectedEof => PromptErrorKind::UserQuit,
            std::io::ErrorKind::InvalidData => PromptErrorKind::UserEmpty,
            _ => PromptErrorKind::StdIn,
        };
        PromptError::Io { kind, source }
    }

    pub fn from_stdout(source: std::io::Error) -> Self {
        let kind = match source.kind() {
            _ => PromptErrorKind::StdOut,
        };
        PromptError::Io { kind, source }
    }
}

pub type Result<T> = std::result::Result<T, PromptError>;
