use super::status::{HttpStatus, HttpStatusErr};

enum Route {
    Root,
    Echo(String),
    UserAgent(String),
    NotFound,
}


impl Route {
    pub fn get_from_request(parsed_buffer: &Vec<String>) -> Self {
        let request = parsed_buffer
            .get(0)
            .unwrap()
            .split_whitespace()
            .collect::<Vec<&str>>();

        // let method = *request.get(0).unwrap();

        let user_agent = parsed_buffer.get(3).unwrap().split(": ").collect::<Vec<&str>>();
        let user_agent = user_agent.get(1).unwrap();

        let path = *request.get(1).unwrap();
        let path = path.split("/").into_iter().collect::<Vec<&str>>();

        let initial_path = path.get(1).unwrap();
        let initial_path = if initial_path.is_empty() {
            "/"
        } else {
            initial_path
        };

        match initial_path {
            "/" => Route::Root,
            "echo" => {
                let echo = path.get(2);

                if let Some(message) = echo {
                    return Route::Echo(String::from(*message));
                }

                Route::NotFound
            },
            "user-agent" => Route::UserAgent(user_agent.to_string()),
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
        let route = Self::get_from_request(&parsed_buffer);

        route
    }
}

pub fn router(buf: &[u8]) -> Result<HttpStatus, HttpStatusErr> {
    let route = Route::get_route_from_buffer(buf);

    match route {
        Route::Root => Ok(HttpStatus::Ok),
        Route::Echo(value) => Ok(HttpStatus::OkWithMessage(value)),
        Route::UserAgent(value)=> Ok(HttpStatus::OkWithMessage(value)),
        Route::NotFound => Err(HttpStatusErr::NotFound),
    }
}
