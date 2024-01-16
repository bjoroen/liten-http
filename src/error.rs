#[derive(Debug)]
pub enum ErrorType {
    ParseError,
}

#[derive(Debug)]
pub struct Error {
    pub error: ErrorType,
    pub error_msg: String,
}
