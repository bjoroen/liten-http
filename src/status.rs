use std::fmt::Display;

pub enum Status {
    Ok,
    SeeOther,
    BadRequest,
    NotFound,
    InternalServerError,
}

impl Status {
    fn status_code(&self) -> usize {
        match self {
            Status::Ok => 200,
            Status::SeeOther => 303,
            Status::BadRequest => 400,
            Status::NotFound => 404,
            Status::InternalServerError => 500,
        }
    }

    fn msg(&self) -> &str {
        match self {
            Status::Ok => "OK",
            Status::SeeOther => "SEE OTHER",
            Status::BadRequest => "BAD REQUEST",
            Status::NotFound => "NOT FOUND",
            Status::InternalServerError => "INTERNAL SERVER ERROR",
        }
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.status_code(), self.msg())
    }
}

#[cfg(test)]
mod test {
    use crate::Status;

    fn codes() -> Vec<(Status, usize, &'static str, &'static str)> {
        vec![
            (Status::Ok, 200, "OK", "200 OK"),
            (Status::SeeOther, 303, "SEE OTHER", "303 SEE OTHER"),
            (Status::BadRequest, 400, "BAD REQUEST", "400 BAD REQUEST"),
            (Status::NotFound, 404, "NOT FOUND", "404 NOT FOUND"),
            (
                Status::InternalServerError,
                500,
                "INTERNAL SERVER ERROR",
                "500 INTERNAL SERVER ERROR",
            ),
        ]
    }

    #[test]
    fn status_code() {
        for (status, code, msg, _) in codes() {
            assert_eq!(status.status_code(), code);
            assert_eq!(status.msg(), msg);
        }
    }

    #[test]
    fn msg() {
        for (status, _, _, to_string) in codes() {
            assert_eq!(status.to_string(), to_string)
        }
    }
}
