use crate::error::{Error, ErrorType};
use crate::Method;

pub struct Request {
    /// A request-line begins with a method token, followed by a single space (SP), the request-target, and another single space (SP),
    /// and ends with the protocol version.
    /// request-line   = method SP request-target SP HTTP-version
    method: Method,
}

impl Request {
    pub fn from_string(request_string: &str) -> Result<Self, Error> {
        Self::parse(request_string)
    }

    fn parse(request_string: &str) -> Result<Request, Error> {
        let parse_error = Error {
            error: ErrorType::ParseError,
            error_msg: "cant parse request".to_string(),
        };

        let parts: Vec<&str> = request_string.split("\r\n").collect();

        let (method, path, version) = match Self::parse_request_line(parts[0]) {
            Ok(v) => v,
            Err(_) => return Err(parse_error),
        };

        dbg!(&method);
        dbg!(path);
        dbg!(version);

        Ok(Request { method })
    }

    fn parse_request_line(request_line: &str) -> Result<(Method, &str, &str), Error> {
        let test: Vec<&str> = request_line.split(" ").collect();
        Ok((Method::from_string(test[0])?, test[1], test[2]))
    }
}

#[cfg(test)]
mod tests {
    use crate::{Method, Request};

    #[test]
    fn parse_method() {
        let request_string = "GET / HTTP/1.1";
        let parsed_method = Request::from_string(request_string).unwrap();

        assert_eq!(parsed_method.method, Method::Get)
    }
}
