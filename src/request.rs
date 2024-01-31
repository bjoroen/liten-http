use std::fmt::Display;

use crate::error::{Error, ErrorType};
use crate::{Header, Method};

#[derive(PartialEq, Eq, Debug)]
/// A request-line begins with a method token, followed by a single space (SP), the request-target, and another single space (SP),
/// and ends with the protocol version.
/// request-line = method SP request-target SP HTTP-version
pub struct Request {
    pub method: Method,
    pub request_target: String,
    //  HTTP-version  = HTTP-name "/" DIGIT "." DIGIT
    //  HTTP-name     = %s"HTTP"
    pub protocol_version: String,
    pub header: Vec<Header>,
    pub body: Option<String>,
}

impl Display for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let body = match &self.body {
            Some(v) => format!("{}\r\n", v),
            None => String::from(""),
        };

        let mut headers = String::new();
        for h in &self.header {
            headers.push_str(&h.to_string());
        }
        write!(
            f,
            "{} {} {}\r\n{}\r\n{}\r\n\r\n",
            self.method, self.request_target, self.protocol_version, headers, body
        )
    }
}

impl Request {
    pub fn from_string(request_string: &str) -> Result<Self, Error> {
        Self::parse(request_string)
    }

    fn parse(request_string: &str) -> Result<Request, Error> {
        let parse_error = Error {
            error: ErrorType::ParseError,
            error_msg: "invalid request format".to_string(),
        };

        let parts: Vec<&str> = request_string.split("\r\n").collect();
        let mut parts_iter = parts.iter();

        let start_line = match parts_iter.next() {
            Some(v) => v,
            None => return Err(parse_error),
        };

        let (method, path, version) = match Self::parse_request_line(start_line) {
            Ok(v) => v,
            Err(_) => return Err(parse_error),
        };

        let mut header = vec![];
        while let Some(h) = parts_iter.next() {
            match *h {
                // Skip empty line between headers and body
                "" => {
                    break;
                }
                _ => header.push(Header::from_field_line(h)?),
            }
        }

        let body = match parts_iter.next() {
            Some(v) => Some(v.to_string()),
            None => None,
        };

        Ok(Request {
            method,
            request_target: String::from(path),
            protocol_version: String::from(version),
            header,
            body,
        })
    }

    fn parse_request_line(request_line: &str) -> Result<(Method, &str, &str), Error> {
        let parse_error = Error {
            error: ErrorType::ParseError,
            error_msg: "invalid request-line format".to_string(),
        };
        let mut parts = request_line.split(" ").into_iter();

        let method = match parts.next() {
            Some(v) => Method::from_string(v)?,
            None => return Err(parse_error),
        };

        let path = match parts.next() {
            Some(v) => v,
            None => return Err(parse_error),
        };

        let version = match parts.next() {
            Some(v) => v,
            None => return Err(parse_error),
        };

        Ok((method, path, version))
    }
}

#[cfg(test)]
mod tests {
    use crate::{Header, Method, Request};

    #[test]
    fn parse_get_request() {
        let request_string = "GET / HTTP/1.1\r\nHost: 127.0.0.1:3000\r\nAccept: */*\r\nContent-Type: application/json\r\nContent-Length: 23";
        let request = Request::from_string(request_string).unwrap();

        assert_eq!(request.method, Method::Get);
        assert_eq!(request.request_target, String::from("/"));
        assert_eq!(request.protocol_version, String::from("HTTP/1.1"));
        assert_eq!(
            request.header[0],
            Header {
                field_name: String::from("Host"),
                field_value: String::from("127.0.0.1:3000")
            }
        );
        assert_eq!(
            request.header[1],
            Header {
                field_name: String::from("Accept"),
                field_value: String::from("*/*")
            }
        );
        assert_eq!(
            request.header[2],
            Header {
                field_name: String::from("Content-Type"),
                field_value: String::from("application/json")
            }
        );
        assert_eq!(
            request.header[3],
            Header {
                field_name: String::from("Content-Length"),
                field_value: String::from("23")
            }
        )
    }

    #[test]
    fn parse_post_request() {
        let request_string = "POST / HTTP/1.1\r\nHost: 127.0.0.1:3000\r\nAccept: */*\r\nContent-Type: application/json\r\nContent-Length: 18\r\n\r\n{\"hello\": \"world\"}";

        let request = Request::from_string(request_string).unwrap();

        assert_eq!(request.method, Method::Post);
        assert_eq!(request.request_target, String::from("/"));
        assert_eq!(request.protocol_version, String::from("HTTP/1.1"));
        assert_eq!(
            request.header[3],
            Header {
                field_name: String::from("Content-Length"),
                field_value: String::from("18")
            }
        );
        assert_eq!(request.body, Some("{\"hello\": \"world\"}".to_string()))
    }
}
