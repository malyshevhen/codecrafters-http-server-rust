use std::fmt::{Display, Formatter};

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
        Self {
            protocol: "HTTP/1.1".to_string(),
            status: StatusCode::Ok,
            content_type,
            content_len: body.len(),
            body: body.to_string(),
        }
    }

    pub fn created(content_type: ContentType) -> Self {
        Self {
            protocol: "HTTP/1.1".to_string(),
            status: StatusCode::Created,
            content_type,
            content_len: 0,
            body: "".to_string(),
        }
    }

    pub fn not_found() -> Self {
        Self {
            protocol: "HTTP/1.1".to_string(),
            status: StatusCode::NotFound,
            content_type: ContentType::TextPlain,
            content_len: 0,
            body: "".to_string(),
        }
    }
}

impl Display for Response {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        let mut resp_string = String::new();
        resp_string.push_str(&format!("{} {}\r\n", self.protocol, self.status,));
        resp_string.push_str(&format!("Content-Type: {}\r\n", self.content_type,));
        resp_string.push_str(&format!("Content-Length: {}\r\n", self.content_len,));
        resp_string.push_str("\r\n");
        resp_string.push_str(&self.body);

        write!(fmt, "{}", resp_string)
    }
}

#[derive(Debug)]
pub enum StatusCode {
    Ok,
    Created,
    NotFound,
}

impl Display for StatusCode {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Self::Ok => "200 OK".to_string(),
            Self::Created => "201 Created".to_string(),
            Self::NotFound => "404 Not Found".to_string(),
        };
        write!(fmt, "{}", str)
    }
}

#[derive(Debug)]
pub enum ContentType {
    TextPlain,
    OctetStream,
}

impl Display for ContentType {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            fmt,
            "{}",
            match self {
                ContentType::TextPlain => "text/plain".to_string(),
                ContentType::OctetStream => "application/octet-stream".to_string(),
            }
        )
    }
}
