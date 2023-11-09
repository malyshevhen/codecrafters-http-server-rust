use std::env;

use crate::http::{Request, Response};
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

async fn read_file_content(directory_path: &str) -> Result<String> {
    let mut file = File::open(directory_path).await?;
    let mut file_content = String::new();
    file.read_to_string(&mut file_content).await?;

    return Ok(file_content);
}

async fn handle_connection(mut stream: TcpStream, directory_path: Option<String>) {
    println!("accepted new connection");

    let mut buf = [0; 512];

    while let Ok(bytes_read) = stream.read(&mut buf).await {
        if bytes_read == 0 {
            break;
        }

        print!("Buffer {}", String::from_utf8_lossy(&buf[..]));

        let request = match Request::deserialize(&buf).await {
            Ok(request) => request,
            Err(e) => {
                eprintln!("Error deserializing request: {}", e);
                continue;
            }
        };
        println!("Receive new request: {}", request.to_string());

        let path = request.path.as_str();
        let response = if path.starts_with("/echo") {
            let dynamic_path = &path["/echo/".len()..];
            Response::ok(dynamic_path, ContentType::TextPlain)
        } else if path.eq("/") {
            Response::ok("", ContentType::TextPlain)
        } else if path.starts_with("/user-agent") {
            let user_agent = request.get_header_value("User-Agent");
            println!("Value of the User-Agent header is: {}", user_agent);
            Response::ok(user_agent, ContentType::TextPlain)
        } else if path.starts_with("/files") {
            let filename = &path["/files/".len()..];
            let full_path = match directory_path {
                Some(ref dir) => format!("{}/{}", dir, filename),
                None => {
                    println!("Directory does not exist.");
                    continue
                },
            };
            if let Ok(file_content) = read_file_content(&full_path).await {
                Response::ok(&file_content, ContentType::OctetStream)
            } else {
                Response::not_found()
            }
        } else {
            Response::not_found()
        };

        if let Err(e) = stream.write(response.to_string().as_bytes()).await {
            println!(
                "Could not write the response to the stream with an Error: {}.",
                e
            );
            break;
        }
    }
}
