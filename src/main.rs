use tokio::io::{AsyncWriteExt, BufReader, Result};
use tokio::net::TcpListener;
use crate::http::{Request, Response};

mod http;

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4221").await?;

    loop {
        let (mut stream, _) = listener.accept().await?;
        println!("accepted new connection");

        let reader = BufReader::new(&mut stream);

        let request = Request::deserialize(reader).await?;
        println!("{:?}", request);

        let path = request.path.as_str();

        let response = if path.starts_with("/echo") {
            Response::ok(path["/echo/".len()..].to_string())
        } else {
            Response::not_found()
        };

        stream.write(response.to_string().as_bytes()).await?;
    }
}
