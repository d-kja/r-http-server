use super::status::{HttpStatus, HttpStatusErr};

enum Route {
    Root,
    NotFound,
}

impl Route {
    pub fn get_from_path(path: &str) -> Self {
        match path {
            "/" => Route::Root,
            _ => Route::NotFound,
        }
    }

    pub fn parse_from_buffer(buf: &[u8]) -> Vec<String> {
        let buf_str = String::from_utf8_lossy(buf);
        let buf_line = buf_str
            .split("\r\n")
            .into_iter()
            .filter(|item| !item.is_empty())
            .map(|item| item.to_owned())
            .collect::<Vec<String>>();

        buf_line.to_owned()
    }

    pub fn get_route_from_buffer(buf: &[u8]) -> Self {
        let parsed_buffer = Self::parse_from_buffer(&buf);
        let request = parsed_buffer
            .get(0)
            .unwrap()
            .split_whitespace()
            .collect::<Vec<&str>>();

        // let method = *request.get(0).unwrap();
        let path = *request.get(1).unwrap();

        let route = Self::get_from_path(path);

        route
    }
}

pub fn router(buf: &[u8]) -> Result<HttpStatus, HttpStatusErr> {
    let route = Route::get_route_from_buffer(buf);

    match route {
        Route::Root => Ok(HttpStatus::Ok),
        Route::NotFound => Err(HttpStatusErr::NotFound),
    }
}
