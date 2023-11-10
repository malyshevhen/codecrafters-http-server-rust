use std::fmt::Display;
use std::io::{Error, ErrorKind};
use std::{collections::HashMap, fmt::Formatter};
use tokio::io::{AsyncBufReadExt, Result};

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub protocol: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl Display for Request {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut resp_string = String::new();
        resp_string.push_str(&format!(
            "{} {} {}\r\n",
            self.method.to_string(),
            self.path.to_string(),
            self.protocol,
        ));
        for (head_title, head_content) in &self.headers {
            resp_string.push_str(&format!("{}: {}\r\n", head_title, head_content));
        }
        resp_string.push_str("\r\n");

        return write!(f, "{}", resp_string);
    }
}

impl Request {
    pub fn new(
        method: Method,
        path: String,
        headers: HashMap<String, String>,
        body: String,
    ) -> Self {
        return Self {
            method,
            path,
            protocol: "HTTP/1.1".to_string(),
            headers,
            body,
        };
    }

    pub async fn deserialize(buf: &[u8]) -> Result<Box<Self>> {
        println!("Parse request...");

        let mut lines = buf.lines();

        let mut method = Method::Get;
        let mut path = String::new();
        let mut headers = HashMap::new();
        let mut body = String::new();

        while let Some(line) = lines.next_line().await? {
            if line.is_empty() {
                continue;
            } else if is_start_line(&line) {
                (method, path) = parse_start_line(&line)?;
                println!("Method: {:?}", &method);
                println!("Path: {}", &path);
                continue;
            } else if is_header(&line) {
                let (header_name, header_value) = parse_header(&line)?;
                println!("Header: {}: {}", &header_name, &header_value);
                headers.insert(header_name, header_value);
                continue;
            } else if !line.is_empty() {
                body = line.trim_end_matches("\x00").to_string();
                println!("Body: {}", &body);
                break;
            } else {
                return Err(Error::new(ErrorKind::Other, "Invalid request"));
            }
        }
        let request = Request::new(method, path, headers, body);
        println!("Parsed request: ");
        println!("{}", request.to_string());

        return Ok(Box::new(request));
    }

    pub fn get_header_value(&self, header_name: &str) -> Option<&str> {
        let header_value = match self.headers.get(header_name) {
            Some(h) => Some(h.as_str()),
            None => None,
        };
        return header_value;
    }
}

fn parse_header(line: &str) -> Result<(String, String)> {
    let mut split_header = line.split_whitespace();
    let key = split_header
        .next()
        .unwrap()
        .trim_end_matches(":")
        .to_string();
    let value = split_header.next().unwrap().to_string();

    return Ok((key, value));
}

fn is_header(line: &str) -> bool {
    line.contains(": ")
}

fn parse_start_line(line: &str) -> Result<(Method, String)> {
    let mut split_start_line = line.split_whitespace();
    let method = match split_start_line.next() {
        Some("GET") => Method::Get,
        Some("POST") => Method::Post,
        Some("PUT") => Method::Put,
        Some("DELETE") => Method::Delete,
        Some(_) => return Err(Error::new(ErrorKind::Other, "Method not supported.")),
        None => {
            return Err(Error::new(
                ErrorKind::Other,
                "Method was not read correctly.",
            ))
        }
    };

    let path = match split_start_line.next() {
        Some(path) => path.to_string(),
        None => panic!("Path was not read correctly."),
    };

    return Ok((method, path));
}

pub fn is_start_line(line: &str) -> bool {
    line.starts_with("GET")
        || line.starts_with("POST")
        || line.starts_with("PUT")
        || line.starts_with("DELETE")
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
    pub fn ok(body: &str, content_type: ContentType) -> Self {
        return Self {
            protocol: "HTTP/1.1".to_string(),
            status: StatusCode::Ok,
            content_type,
            content_len: body.len(),
            body: body.to_string(),
        };
    }

    pub fn created(content_type: ContentType) -> Self {
        return Self {
            protocol: "HTTP/1.1".to_string(),
            status: StatusCode::Created,
            content_type,
            content_len: 0,
            body: "".to_string(),
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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
        resp_string.push_str(&format!("Content-Length: {}\r\n", self.content_len,));
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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
    Created,
    NotFound,
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Self::Ok => "200 OK".to_string(),
            Self::Created => "201 Created".to_string(),
            Self::NotFound => "404 Not Found".to_string(),
        };
        return write!(f, "{}", str);
    }
}

#[derive(Debug)]
pub enum ContentType {
    TextPlain,
    OctetStream,
}

impl Display for ContentType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return write!(
            f,
            "{}",
            match self {
                ContentType::TextPlain => "text/plain".to_string(),
                ContentType::OctetStream => "application/octet-stream".to_string(),
            }
        );
    }
}
