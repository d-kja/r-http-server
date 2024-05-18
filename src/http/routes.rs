use std::{env, fs};

use super::status::{HttpStatus, HttpStatusErr};

enum Route {
    Root,
    Echo(String),
    UserAgent(String),
    Files(String),
    NotFound,
}

impl Route {
    pub fn get_from_request(parsed_buffer: &Vec<String>) -> Self {
        let request = parsed_buffer
            .get(0)
            .unwrap()
            .split_whitespace()
            .collect::<Vec<&str>>();


        dbg!("DEBUG:", &parsed_buffer);

        // let method = *request.get(0).unwrap();
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
            "user-agent" => {
                // Well... my curl has a Accept header =D
                let user_agent = if parsed_buffer.get(2).expect("second header not found").starts_with("Accept:") {
                    parsed_buffer.get(3).expect("third header not found").split(": ").collect::<Vec<&str>>()
                } else {
                    parsed_buffer.get(2).expect("second header not found").split(": ").collect::<Vec<&str>>()
                };

                let user_agent = user_agent.get(1).expect("user agent not found");

                Route::UserAgent(user_agent.to_string())
            },
            "files" => {
                let file = path.get(2);
                let args = env::args();

                let folder = args.last().unwrap_or(String::from("./")); 

                if let Some(path) = file {
                    let file = fs::read_to_string(format!("{folder}{path}"));

                    if file.is_err() {
                        return Route::NotFound;
                    }

                    return Route::Files(file.expect("shouldn't throw!"));
                }

                Route::NotFound
            }
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
        Route::Files(value)=> Ok(HttpStatus::OkWithFile(value)),
        Route::NotFound => Err(HttpStatusErr::NotFound),
    }
}
