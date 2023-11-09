use crate::http::{Request, Response};
use tokio::io::{AsyncReadExt, AsyncWriteExt, Result};
use tokio::net::TcpListener;

mod http;

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4221").await?;

    while let Ok((mut stream, _)) = listener.accept().await {
        tokio::spawn(async move {
            println!("accepted new connection");

            let mut buf = [0; 512];

            loop {
                match stream.read(&mut buf).await {
                    Ok(b) => {
                        if b == 0 {
                            break b;
                        } else {
                            b
                        }
                    }
                    Err(_) => panic!("Could not read from stream."),
                };

                print!("Buffer {}", String::from_utf8_lossy(&buf[..]));

                let request = match Request::deserialize(&buf).await {
                    Ok(request) => request,
                    Err(e) => panic!("{}", e),
                };
                println!("Receive new request: {}", request.to_string());

                let path = request.path.as_str();

                let response = if path.starts_with("/echo") {
                    Response::ok(&path["/echo/".len()..])
                } else if path.eq("/") {
                    Response::ok("")
                } else if path.starts_with("/user-agent") {
                    let body = request.get_header_value("User-Agent");
                    println!("Value of the User-Agent header is: {}", body);

                    Response::ok(body)
                } else {
                    Response::not_found()
                };

                match stream.write(response.to_string().as_bytes()).await {
                    Ok(b) => b,
                    Err(_) => panic!("Could not write the response to the stream."),
                };
            }
        });
    }
    Ok(())
}
