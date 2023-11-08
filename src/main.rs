use std::{io::Write, net::TcpListener};
use std::io::{BufRead, BufReader};

use nom::AsBytes;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");

                let mut reader = BufReader::new(&mut stream);

                let mut line = String::new();
                reader.read_line(&mut line).expect("Could not read request");

                let mut first_line_iter = line.split_whitespace();
                let _http_method = first_line_iter.next().unwrap();
                let path = first_line_iter.next().unwrap();

                let ok = "HTTP/1.1 200 OK\r\n\r\n";
                let not_found = "HTTP/1.1 404 Not Found\r\n\r\n";

                let response = match path {
                    "/" => ok,
                    &_ => not_found,
                };

                let res = stream.write(response.as_bytes());
                match res {
                    Ok(_) => println!("Response is {}", response),
                    Err(e) => println!("error: {}", e),
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
