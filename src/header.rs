use crate::{Error, ErrorType};

/// field-line   = field-name ":" OWS field-value OWS
#[derive(Debug, PartialEq, Eq)]
pub struct Header {
    field_name: String,
    field_value: String,
}

impl Header {
    /// Creates a header from a key and a value
    ///
    /// # Example
    /// ```
    /// let content_length = Header{field_name: "Content-Length", field_value: "10"};
    ///
    /// assert_eq!(Header::new("Content-Length", "10"), content_length)
    /// ```
    pub fn new(field_name: &str, field_value: &str) -> Header {
        Header {
            field_name: field_name.to_string(),
            field_value: field_value.to_string(),
        }
    }

    /// Creates a header from a header field-line string
    /// field-line = field-name ":" OWS field-value OWS
    /// e.g Content-Type: application/json
    ///
    /// # Example
    /// ```
    /// let content_type = Header{field_name: "Content-Length", field_value: "10"};
    /// let field_line = "Content-Length: 23"
    ///
    /// assert_eq!(Header::from_field_line(field_line.to_string()), content_type)
    /// ```
    pub fn from_field_line(header: &str) -> Result<Header, Error> {
        let error = Error {
            error: ErrorType::ParseError,
            error_msg: "invalid header format".to_string(),
        };
        let (field_name, field_value) = match header.split_once(':') {
            Some(v) => v,
            None => return Err(error),
        };

        // field-values have a OWS at the start and at the end of the field, remove it if its
        // there to not include it in the header struct
        let mut field_value_no_ows = field_value.chars().collect::<Vec<char>>();
        if !field_value_no_ows.is_empty() {
            if field_value_no_ows[0].is_whitespace() {
                field_value_no_ows.remove(0);
            }

            if field_value_no_ows.last().is_some_and(|f| f.is_whitespace()) {
                field_value_no_ows.pop();
            }
        }

        let field_value = String::from_iter(field_value_no_ows);

        Ok(Header {
            field_name: field_name.to_string(),
            field_value: field_value.to_string(),
        })
    }

    /// Creates a header from a header section string
    ///
    /// # Example
    /// ```
    /// let header_section = r#"Host: 127.0.0.1:3000
    /// Content-Type: application/json
    /// Content-Length: 23"#;
    ///
    ///
    /// let expected_headers = vec![
    ///     Header {
    ///         field_name: "Host".to_string(),
    ///         field_value: "127.0.0.1:3000".to_string(),
    ///     },
    ///     Header {
    ///         field_name: "Content-Type".to_string(),
    ///         field_value: "application/json".to_string(),
    ///     },
    ///     Header {
    ///         field_name: "Content-Length".to_string(),
    ///         field_value: "23".to_string(),
    ///     },
    /// ];
    ///
    /// assert_eq!(Header::from_section(header_section.to_string()), expected_headers)
    /// ```
    pub fn from_section(header_section: String) -> Result<Vec<Header>, Error> {
        let field_lines = header_section.split("\n");

        let mut headers: Vec<Header> = vec![];
        for line in field_lines {
            match Self::from_field_line(line) {
                Ok(v) => headers.push(v),
                Err(e) => return Err(e),
            }
        }

        Ok(headers)
    }
}

#[cfg(test)]
mod test {
    use crate::{ErrorType, Header};

    #[test]
    fn header_new() {
        let header = Header::new("Content-Length", "69");
        assert_eq!(
            header,
            Header {
                field_name: "Content-Length".to_string(),
                field_value: "69".to_string()
            }
        )
    }

    #[test]
    fn header_from_field_line() {
        let header_string = "Host: 127.0.0.1:3000";
        let header = Header::from_field_line(header_string).unwrap();

        assert_eq!(
            header,
            Header {
                field_name: "Host".to_string(),
                field_value: "127.0.0.1:3000".to_string()
            }
        );

        let bad_format_header = "Content-Length 23
";

        let header = Header::from_field_line(bad_format_header).unwrap_err();

        assert_eq!(header.error, ErrorType::ParseError)
    }

    #[test]
    fn header_from_header_section() {
        let header_section = r#"Host: 127.0.0.1:3000
Accept: */*
Content-Type: application/json
Content-Length: 23"#;

        let expected_headers = vec![
            Header {
                field_name: "Host".to_string(),
                field_value: "127.0.0.1:3000".to_string(),
            },
            Header {
                field_name: "Accept".to_string(),
                field_value: "*/*".to_string(),
            },
            Header {
                field_name: "Content-Type".to_string(),
                field_value: "application/json".to_string(),
            },
            Header {
                field_name: "Content-Length".to_string(),
                field_value: "23".to_string(),
            },
        ];

        let headers = Header::from_section(header_section.to_string()).unwrap();
        for (i, item) in headers.iter().enumerate() {
            assert_eq!(item, &expected_headers[i])
        }
    }
}
