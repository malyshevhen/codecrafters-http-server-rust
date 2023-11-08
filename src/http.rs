use std::fmt::Display;
use tokio::io::{AsyncBufReadExt, BufReader, Result};
use tokio::net::TcpStream;

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub protocol: String,
}

impl Request {
    pub fn new(method: Method, path: String) -> Self {
        return Self {
            method,
            path,
            protocol: "HTTP/1.1".to_string(),
        };
    }

    pub async fn deserialize(reader: BufReader<&mut TcpStream>) -> Result<Box<Self>> {
        let mut lines = reader.lines();
        let header_iter = lines.next_line().await?.unwrap();

        let mut split_header = header_iter.split_whitespace();
        let method = match split_header.next().unwrap() {
            "GET" => Method::Get,
            "POST" => Method::Post,
            "PUT" => Method::Put,
            "DELETE" => Method::Delete,
            _ => panic!(),
        };
        let path = split_header.next().unwrap().to_string();

        return Ok(Box::new(Self::new(method, path)));
    }
}

#[derive(Debug)]
pub struct Response {
    pub protocol: String,
    pub status: StatusCode,
    pub content_type: ContentType,
    pub content_len: usize,
    pub body: String,
}

impl Response {
    pub fn ok(body: String) -> Self {
        return Self {
            protocol: "HTTP/1.1".to_string(),
            status: StatusCode::Ok,
            content_type: ContentType::TextPlain,
            content_len: body.len(),
            body,
        };
    }

    pub fn not_found() -> Self {
        return Self {
            protocol: "HTTP/1.1".to_string(),
            status: StatusCode::NotFound,
            content_type: ContentType::TextPlain,
            content_len: 0,
            body: "".to_string(),
        };
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut resp_string = String::new();
        resp_string.push_str(&format!(
            "{} {}\r\n",
            self.protocol.to_string(),
            self.status.to_string(),
        ));
        resp_string.push_str(&format!(
            "Content-Type: {}\r\n",
            self.content_type.to_string(),
        ));
        resp_string.push_str(&format!(
            "Content-Length: {}\r\n",
            self.content_len,
        ));
        resp_string.push_str("\r\n");
        resp_string.push_str(&self.body);

        return write!(f, "{}", resp_string);
    }
}

#[derive(Debug)]
pub enum Method {
    Get,
    Put,
    Post,
    Delete,
}

impl Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Method::Get => "GET".to_string(),
            Method::Put => "PUT".to_string(),
            Method::Post => "POST".to_string(),
            Method::Delete => "DELETE".to_string(),
        };
        write!(f, "{}", str)
    }
}

#[derive(Debug)]
pub enum StatusCode {
    Ok,
    NotFound,
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            StatusCode::Ok => "200 OK".to_string(),
            StatusCode::NotFound => "404 Not Found".to_string(),
        };
        return write!(f, "{}", str);
    }
}

#[derive(Debug)]
pub enum ContentType {
    TextPlain
}

impl Display for ContentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", match self {
            ContentType::TextPlain => "text/plain".to_string()
        });
    }
}