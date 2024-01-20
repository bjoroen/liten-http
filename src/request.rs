use crate::error::{Error, ErrorType};
use crate::{Header, Method};

#[derive(PartialEq, Eq, Debug)]
/// A request-line begins with a method token, followed by a single space (SP), the request-target, and another single space (SP),
/// and ends with the protocol version.
/// request-line = method SP request-target SP HTTP-version
pub struct Request {
    method: Method,
    request_target: String,
    //  HTTP-version  = HTTP-name "/" DIGIT "." DIGIT
    //  HTTP-name     = %s"HTTP"
    protocol_version: String,
    header: Vec<Header>,
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

        let (method, path, version) = match Self::parse_request_line(parts[0]) {
            Ok(v) => v,
            Err(_) => return Err(parse_error),
        };

        dbg!(&parts[1]);

        let header = Header::from_section(parts[1].to_string())?;

        Ok(Request {
            method,
            request_target: String::from(path),
            protocol_version: String::from(version),
            header,
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
    use crate::{Method, Request};

    #[test]
    fn parse_get_request() {
        let request_string = "GET / HTTP/1.1\r\nHost: 127.0.0.1:3000\r\nAccept: */*\r\nContent-Type: application/json\r\nContent-Length: 23";
        let request = Request::from_string(request_string).unwrap();

        assert_eq!(request.method, Method::Get);
        assert_eq!(request.request_target, String::from("/"));
        assert_eq!(request.protocol_version, String::from("HTTP/1.1"));
    }
}
