use std::convert::{From, Into};
use std::io::BufRead;
use std::io::BufReader;
use std::net::TcpStream;

#[derive(Clone, Debug)]
pub struct HttpResponse {
    pub version: String,
    pub status_code: String,
    pub status_statement: String,
    pub headers: String,
    pub data: String,
}

impl Into<String> for HttpResponse {
    fn into(self) -> String {
        format!(
            "HTTP/{} {} {}\n{}\n\n{}",
            self.version, self.status_code, self.status_statement, self.headers, self.data
        )
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum HttpMethod {
    GET,
    POST,
    PATCH,
    DELETE,
    OPTION,
}

#[derive(Clone, Debug)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub path: String,
    pub version: String,
    pub headers: String,
    pub data: String,
}

impl From<&TcpStream> for HttpRequest {
    fn from(stream: &TcpStream) -> Self {
        let mut stream = BufReader::new(stream);
        let mut first_line = String::new();
        if let Err(err) = stream.read_line(&mut first_line) {
            panic!("error during receive a line: {}", err);
        }
        let mut params = first_line.split_whitespace();
        let method = params.next();
        let path = params.next();
        match (method, path) {
            (Some("GET"), Some(file_path)) => HttpRequest {
                method: HttpMethod::GET,
                path: String::from(file_path),
                version: String::from("1.1"),
                headers: String::from(""),
                data: String::from(""),
            },
            _ => panic!("failed to parse"),
        }
    }
}
