use std::fmt::Display;

use crate::{Header, Status};

pub struct Response {
    protocol_version: String,
    status: Status,
    header: Vec<Header>,
    body: Option<String>,
}

impl Default for Response {
    fn default() -> Self {
        Self {
            protocol_version: String::from("HTTP/1.1"),
            status: Status::Ok,
            header: Default::default(),
            body: Default::default(),
        }
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut headers = String::new();
        for h in &self.header {
            headers.push_str(&h.to_string())
        }
        let body = match &self.body {
            Some(v) => v.to_owned(),
            None => String::from(""),
        };
        write!(
            f,
            "{} {}\r\n{}\r\n{}",
            self.protocol_version, self.status, headers, body
        )
    }
}

impl Response {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn status(self, status: Status) -> Self {
        Response { status, ..self }
    }

    pub fn header(self, h: Header) -> Self {
        let mut header = self.header;
        header.push(h);
        Response { header, ..self }
    }

    pub fn content(self, content: &str, content_type: &str) -> Response {
        let content_length_header = Header::new("Content-Length", &content.len().to_string());
        let content_type_header = Header::new("Content-Type", content_type);
        let mut header = vec![];
        header.push(content_length_header);
        header.push(content_type_header);

        Self {
            header,
            body: Some(content.to_string()),
            ..self
        }
    }

    pub fn html(self, content: &str) -> Self {
        Self::content(self, content, "text/html")
    }

    pub fn json(self, content: &str) -> Self {
        Self::content(self, content, "application/json")
    }
}

#[cfg(test)]
mod test {
    use crate::{Header, Response, Status};

    #[test]
    fn response_without_body() {
        let response = Response::new().status(Status::Ok);

        assert_eq!(response.to_string(), "HTTP/1.1 200 OK\r\n\r\n")
    }

    #[test]
    fn response_with_headers() {
        let response = Response::new()
            .status(Status::Ok)
            .header(Header::new("Content-Length", "69"));

        assert_eq!(
            response.to_string(),
            "HTTP/1.1 200 OK\r\nContent-Length: 69\r\n\r\n"
        )
    }

    #[test]
    fn response_with_html_body() {
        let response = Response::new()
            .status(Status::Ok)
            .html("<HTML><h1>Hello World</h1></HTML>");

        assert_eq!(
            response.to_string(),
            "HTTP/1.1 200 OK\r\nContent-Length: 33\r\nContent-Type: text/html\r\n\r\n<HTML><h1>Hello World</h1></HTML>"
        )
    }

    #[test]
    fn response_with_json_body() {
        let response = Response::new()
            .status(Status::Ok)
            .json("{\"hello\": \"world\"}");

        assert_eq!(
            response.to_string(),
            "HTTP/1.1 200 OK\r\nContent-Length: 18\r\nContent-Type: application/json\r\n\r\n{\"hello\": \"world\"}"
        )
    }
}
