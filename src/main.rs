use crate::error::Result;
use crate::http::handlers::handle_request;
use crate::http::request::Request;
use config::config;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

mod config;
mod error;
mod http;
pub mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4221").await?;
    println!("{:<12} - main - Server Start On Port: 4221", "MAIN");

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_connection(stream));
    }

    return Ok(());
}

async fn handle_connection(mut stream: TcpStream) -> Result<()> {
    println!("{:<12} - handle_connection - Receive The Connection.", "MAIN");

    let mut buf = [0; 512];

    while let Ok(bytes_read) = stream.read(&mut buf).await {
        if bytes_read == 0 {
            break;
        }
        let request = Request::parse_from_bytes(&buf).await?;
        let response = handle_request(request).await?;
        stream.write_all(response.to_string().as_bytes()).await?;
    }

    println!("{:<12} - handle_connection - Close The Connection.", "MAIN");

    Ok(())
}
