use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum HttpStatus {
    Ok,
    OkWithMessage(String),
    OkWithFileRead(String),
        OkWithFileWrite,
    NotFound,
}

#[non_exhaustive]
#[derive(Debug)]
pub enum HttpStatusErr {
    NotFound,
}

impl Display for HttpStatusErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpStatusErr::NotFound => write!(f, "{}", HttpStatus::NotFound.get_response()),
        }
    }
}

const PREFIX_RESPONSE: &str = "HTTP/1.1";
const BREAK_LINE: &str = "\r\n";
const DOUBLE_BREAK_LINE: &str = "\r\n\r\n";

impl HttpStatus {
    pub fn get_response(&self) -> String {
        match self {
            Self::Ok => format!("{PREFIX_RESPONSE} 200 OK{DOUBLE_BREAK_LINE}"),
            Self::OkWithMessage(value) => format!("{PREFIX_RESPONSE} 200 OK{BREAK_LINE}Content-Type: text/plain{BREAK_LINE}Content-Length: {}{DOUBLE_BREAK_LINE}{value}", value.len()),
            Self::OkWithFileRead(value) => format!("{PREFIX_RESPONSE} 200 OK{BREAK_LINE}Content-Type: application/octet-stream{BREAK_LINE}Content-Length: {}{DOUBLE_BREAK_LINE}{value}", value.len()),
            Self::OkWithFileWrite => format!("{PREFIX_RESPONSE} 201 Created{DOUBLE_BREAK_LINE}"),
            Self::NotFound => format!("{PREFIX_RESPONSE} 404 Not Found{DOUBLE_BREAK_LINE}"),
        }
    }
}

impl HttpStatusErr {
    pub fn get_response(&self) -> String {
        match self {
            Self::NotFound => HttpStatus::NotFound.get_response(),
        }
    }
}
