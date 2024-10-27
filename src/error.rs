use std::io::ErrorKind;
use std::fmt;

#[derive(Debug)]
pub enum ErrorType {
    ParseError,
    IoError,
    OtherError,
}

pub struct Errors {
    error_type: ErrorType,
    message: String,
}

impl Errors {
    pub fn new(error_type: ErrorType, message: impl Into<String>) -> Self {
        Errors {
            error_type,
            message: message.into(),
        }
    }
}

// Implement `From` for standard I/O errors
impl From<std::io::Error> for Errors {
    fn from(err: std::io::Error) -> Self {
        Errors::new(ErrorType::IoError, err.to_string())
    }
}

// Implement `From<&str>` for convenient string conversion
impl From<&str> for Errors {
    fn from(message: &str) -> Self {
        Errors::new(ErrorType::OtherError, message)
    }
}

impl fmt::Debug for Errors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {}", self.error_type, self.message)
    }
}

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", match self.error_type {
            ErrorType::ParseError => "Parse Error",
            ErrorType::IoError => "IO Error",
            ErrorType::OtherError => "Other Error",
        }, self.message)
    }
}
