use std::io;
use rustyline::error::ReadlineError;
use std::fmt;

pub enum PassError {
    Interrupt,
    WrongPassword,
    NotFound,
    Other(String),
}

impl From<io::Error> for PassError {
    fn from(error: io::Error) -> Self {
        match error.kind() {
            io::ErrorKind::NotFound => PassError::NotFound,
            _ => PassError::Other(error.to_string()),
        }
    }
}

impl From<ReadlineError> for PassError {
    fn from(error: ReadlineError) -> Self {
        match &error {
            ReadlineError::Interrupted | ReadlineError::Eof => PassError::Interrupt,
            err => PassError::Other(error.to_string()),
        }
    }
}

impl From<String> for PassError {
    fn from(error: String) -> Self {
        PassError::Other(error.to_string())
    }
}

impl From<serde_json::Error> for PassError {
    fn from(error: serde_json::Error) -> Self {
        PassError::Other(error.to_string())
    }
}

impl fmt::Display for PassError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PassError::Interrupt => write!(f, "Keyboard Interrupt"),
            PassError::WrongPassword => write!(f, "Wrong Password"),
            PassError::NotFound => write!(f, "Not Found"),
            PassError::Other(msg) => write!(f, "{}", msg),
        }
    }
}
