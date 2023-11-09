use std::collections::HashMap;
use std::fmt::Display;
use tokio::io::{AsyncBufReadExt, Result};

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub protocol: String,
    pub headers: HashMap<String, String>,
}

impl Display for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
    pub fn new(method: Method, path: String, headers: HashMap<String, String>) -> Self {
        return Self {
            method,
            path,
            protocol: "HTTP/1.1".to_string(),
            headers,
        };
    }

    pub async fn deserialize(buf: &[u8]) -> Result<Box<Self>> {
        println!("Parse request...");
        println!("Incoming Buffer {:?}", buf);
        
        let mut lines = buf.lines();

        let start_line = match lines.next_line().await? {
            Some(start_line) => start_line,
            None => panic!("Start Line was not read correctly."),
        };
        println!("Unparsed Start Line: {}", start_line);

        let mut split_start_line = start_line.split_whitespace();
        let method = match split_start_line.next()
         {
            Some("GET") => Method::Get,
            Some("POST") => Method::Post,
            Some("PUT") => Method::Put,
            Some("DELETE") => Method::Delete,
            Some(_) => panic!("Method not supported."),
            None => panic!("Method was not read correctly."),
        };

        println!("Request method is: {:?}", method);

        let path = match split_start_line.next() {
            Some(path) => path.to_string(),
            None => panic!("Path was not read correctly."),
        };

        println!("Requested path: {}", path);
        println!("{}", "=".repeat(40));
        // println!("Unparsed request tail is: {:?}", &lines);

        let mut headers = HashMap::new();

        println!("Parse request Headers...");

        loop {
            let line = match lines.next_line().await? {
                Some(l) => {
                    if l.is_empty() {

                        println!("Last header was parsed.");
                        
                        break;
                    }

                    println!("Parsed header: {}", &l);
                    
                    l
                }
                None => break,
            };
            
            println!("Header: {}", line);

            let mut split_header = line.split_whitespace();
            let key = split_header.next().unwrap().to_string();
            let value = split_header.next().unwrap().to_string();
            headers.insert(key, value);
        }

        println!("{}", "=".repeat(40));
        // println!("Unparsed request tail is: {:?}", &lines);

        return Ok(Box::new(Self::new(method, path, headers)));
    }

    pub fn get_header_value(&self, header_name: &str) -> &str {
        let header_value = match self.headers.get(header_name) {
            Some(h) => h,
            None => "",
        };
        return header_value;
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
    pub fn ok(body: &str) -> Self {
        return Self {
            protocol: "HTTP/1.1".to_string(),
            status: StatusCode::Ok,
            content_type: ContentType::TextPlain,
            content_len: body.len(),
            body: body.to_string(),
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
    TextPlain,
}

impl Display for ContentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(
            f,
            "{}",
            match self {
                ContentType::TextPlain => "text/plain".to_string(),
            }
        );
    }
}
