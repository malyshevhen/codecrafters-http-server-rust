use std::env;
use std::io::{Error, ErrorKind};

use crate::http::{Method, Request, Response};
use http::ContentType;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt, Result};
use tokio::net::{TcpListener, TcpStream};

mod http;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let directory_path = args
        .iter()
        .position(|arg| arg == "--directory")
        .and_then(|index| args.get(index + 1))
        .map(|st| st.to_string());

    let listener = TcpListener::bind("127.0.0.1:4221").await?;

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_connection(stream, directory_path.clone()));
    }

    return Ok(());
}

async fn handle_connection(mut stream: TcpStream, directory_path: Option<String>) -> Result<()> {
    println!("accepted new connection");

    let mut buf = [0; 512];

    while let Ok(bytes_read) = stream.read(&mut buf).await {
        if bytes_read == 0 {
            break;
        }

        let request = Request::deserialize(&buf).await?;

        let path = request.path.as_str();
        let response: Response = if path.eq("/") {
            get_simple_ok()
        } else if path.starts_with("/echo") {
            get_echo_response(&path)
        } else if path.starts_with("/user-agent") {
            get_user_agent_response(&request)
        } else if path.starts_with("/files") {
            match request.method {
                Method::Get => get_file_sent_response(&path, &directory_path).await?,
                Method::Post => get_file_saved_response(&path, &directory_path, &request.body).await?,
                _ => Response::not_found(),
            }
        } else {
            Response::not_found()
        };
        stream.write(response.to_string().as_bytes()).await?;
    }
    Ok(())
}

async fn get_file_saved_response(path: &str, directory_path: &Option<String>, file_content: &str) -> Result<Response> {
    let filename = &path["/files/".len()..];
    let full_path = match directory_path {
        Some(ref dir) => format!("{}/{}", dir, filename),
        None => return Err(Error::new(ErrorKind::Other, "Directory does not exist.")),
    };

    let mut file = File::create(full_path).await?;
    file.write_all(&file_content.as_bytes()).await?;

    return Ok(Response::created(ContentType::OctetStream));
}

async fn get_file_sent_response(path: &str, directory_path: &Option<String>) -> Result<Response> {
    let filename = &path["/files/".len()..];
    let full_path = match directory_path {
        Some(ref dir) => format!("{}/{}", dir, filename),
        None => return Err(Error::new(ErrorKind::Other, "Directory does not exist.")),
    };
    if let Ok(file_content) = read_file_content(&full_path).await {
        let response = Response::ok(&file_content, ContentType::OctetStream);
        return Ok(response);
    } else {
        let response = Response::not_found();
        return Ok(response);
    }
}

async fn read_file_content(directory_path: &str) -> Result<String> {
    let mut file = File::open(directory_path).await?;
    let mut file_content = String::new();
    file.read_to_string(&mut file_content).await?;

    return Ok(file_content);
}


fn get_user_agent_response(request: &Request) -> Response {
    if let Some(user_agent) = request.get_header_value("User-Agent") {
        Response::ok(user_agent, ContentType::TextPlain)
    } else {
        Response::not_found()
    }
}

fn get_echo_response(path: &str) -> Response {
    let dynamic_path = &path["/echo/".len()..];
    return Response::ok(dynamic_path, ContentType::TextPlain);
}

fn get_simple_ok() -> Response {
    return Response::ok("", ContentType::TextPlain);
}
