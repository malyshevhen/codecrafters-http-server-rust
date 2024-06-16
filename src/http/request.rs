use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
};
use tokio::io::AsyncBufReadExt;

use crate::http::error::{Error, Result};

#[derive(Debug, Clone)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub protocol: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl Display for Request {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        let mut resp_string = String::new();
        resp_string.push_str(&format!(
            "{} {} {}\r\n",
            self.method, self.path, self.protocol,
        ));
        for (head_title, head_content) in &self.headers {
            resp_string.push_str(&format!("{}: {}\r\n", head_title, head_content));
        }
        resp_string.push_str("\r\n");

        write!(fmt, "{}", resp_string)
    }
}

impl Request {
    pub async fn parse_from_bytes(buf: &[u8]) -> Result<Self> {
        println!("{:<12} - parse_from_bytes -", "REQUEST");

        let mut lines = buf.lines();

        let mut request_builder = RequestBuilder::new();

        while let Some(line) = lines.next_line().await? {
            if line.is_empty() {
                continue;
            } else if is_start_line(&line) {
                let (method, path) = parse_start_line(&line)?;
                request_builder = request_builder.method(method).path(path);
                continue;
            } else if is_header(&line) {
                let header = parse_header(&line)?;
                request_builder = request_builder.header(header);
                continue;
            } else if !line.is_empty() {
                let body = line.trim_end_matches('\x00').to_string();
                request_builder = request_builder.body(body);
                break;
            } else {
                return Err(Error::InvalidRequest);
            }
        }
        let request = request_builder.build()?;
        println!("{:<12} - Parsed request: \n{}", "REQUEST",request);

        Ok(request)
    }

    pub fn get_header_value(&self, header_name: &str) -> Option<&str> {
        self.headers.get(header_name).map(|h| h.as_str())
    }
}

fn parse_header(line: &str) -> Result<(String, String)> {
    let mut split_header = line.split_whitespace();
    if let Some(key) = split_header.next() {
        let key = key.trim_end_matches(':').to_string();
        let value = split_header.next().unwrap_or_default().to_string();

        return Ok((key, value));
    };
    Err(Error::CanNotParseHeader)
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
        Some(_) => return Err(Error::MethodNotSupported),
        None => return Err(Error::MethodWasNotReadCorrectly),
    };

    let path = match split_start_line.next() {
        Some(path) => path.to_string(),
        None => panic!("Path was not read correctly."),
    };

    Ok((method, path))
}

pub fn is_start_line(line: &str) -> bool {
    line.starts_with("GET")
        || line.starts_with("POST")
        || line.starts_with("PUT")
        || line.starts_with("DELETE")
}

#[derive(Debug, Clone)]
pub enum Method {
    Get,
    Put,
    Post,
    Delete,
}

impl Display for Method {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Method::Get => "GET".to_string(),
            Method::Put => "PUT".to_string(),
            Method::Post => "POST".to_string(),
            Method::Delete => "DELETE".to_string(),
        };
        write!(fmt, "{}", str)
    }
}

pub struct RequestBuilder {
    pub method: Option<Method>,
    pub path: Option<String>,
    pub protocol: Option<String>,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

impl Default for RequestBuilder {
    fn default() -> Self {
        Self {
            method: Default::default(),
            path: Default::default(),
            protocol: Some("HTTP/1.1".to_string()),
            headers: Default::default(),
            body: Default::default(),
        }
    }
}

impl RequestBuilder {
    pub fn new() -> Self {
        RequestBuilder::default()
    }

    pub fn method(mut self, method: Method) -> Self {
        self.method = Some(method);
        self
    }

    pub fn path(mut self, path: impl Into<String>) -> Self {
        self.path = Some(path.into());
        self
    }

    pub fn header(mut self, header: (impl Into<String>, impl Into<String>)) -> Self {
        let (k, v) = header;
        self.headers.insert(k.into(), v.into());
        self
    }

    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }

    pub fn build(self) -> Result<Request> {
        let method = self.method.unwrap_or(Method::Get);
        let Some(path) = self.path else {
            return Err(Error::NoUrl);
        };
        let protocol = self.protocol.unwrap_or_else(|| "HTTP/1.1".to_string());
        let body = self.body.unwrap_or_default();

        Ok(Request {
            method,
            path,
            protocol,
            headers: self.headers.clone(),
            body,
        })
    }
}
