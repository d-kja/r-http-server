#[derive(Debug)]
pub enum HttpStatus {
    Ok,
}

const PREFIX_RESPONSE: &str = "HTTP/1.1";
const BREAK_LINE: &str = "\r\n\r\n";

impl HttpStatus {
    pub fn get_response(&self) -> String {
        match self {
            Self::Ok => format!("{PREFIX_RESPONSE} 200 OK{BREAK_LINE}"),
        }
    }
}
