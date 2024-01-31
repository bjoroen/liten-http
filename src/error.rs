use std::{fmt::Display, io};

#[derive(Debug, PartialEq, Eq)]
pub enum ErrorType {
    ParseError,
}

#[derive(Debug)]
pub struct Error {
    pub error: ErrorType,
    pub error_msg: String,
}

impl Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorType::ParseError => write!(f, "{}", String::from("PARSER ERROR")),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.error, self.error_msg)
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Error {
            error: ErrorType::ParseError,
            error_msg: value.to_string(),
        }
    }
}
