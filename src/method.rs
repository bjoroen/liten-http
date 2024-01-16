use crate::error::{Error, ErrorType};

#[derive(Debug, PartialEq, Eq)]
pub enum Method {
    Get,
    Post,
    Delete,
    Put,
}

impl Method {
    pub fn from_string(method_string: &str) -> Result<Self, Error> {
        let parse_error = Error {
            error_msg: "Unknown method".to_string(),
            error: ErrorType::ParseError,
        };
        match method_string {
            "GET" => Ok(Method::Get),
            "POST" => Ok(Method::Post),
            "DELETE" => Ok(Method::Delete),
            "PUT" => Ok(Method::Put),
            _ => Err(parse_error),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Method;

    #[test]
    fn from_string() {
        let method_string = vec![
            ("GET", Method::Get),
            ("POST", Method::Post),
            ("DELETE", Method::Delete),
            ("PUT", Method::Put),
        ];

        for (string, method) in method_string {
            assert_eq!(Method::from_string(string).unwrap(), method)
        }
    }
}
