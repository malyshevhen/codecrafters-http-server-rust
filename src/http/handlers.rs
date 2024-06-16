use crate::config;
use crate::error::{Error, Result};
use crate::http::request::Request;
use crate::http::{ContentType, Method, Response};
use crate::utils::{read_file_content, write_file};

pub async fn handle_request(request: Request) -> Result<Response> {
    println!("{:<12} - handle_request", "HANDLERS");

    let path_tokens = request.path.split('/').collect::<Vec<&str>>();

    if let Some(root_path) = path_tokens.get(1).copied() {
        let response = match root_path {
            "" => home_handler(),
            "echo" => echo_handler(request),
            "user-agent" => user_agent_handler(request),
            "files" => file_path_handler(request).await?,
            _ => Response::not_found(),
        };
        return Ok(response);
    };

    Err(Error::InvalidRequest)
}

fn user_agent_handler(request: Request) -> Response {
    println!("{:<12} - user_agent_handler", "HANDLERS");

    request
        .get_header_value("User-Agent")
        .map_or(Response::not_found(), |user_agent| {
            Response::ok(user_agent, ContentType::TextPlain)
        })
}

fn echo_handler(request: Request) -> Response {
    println!("{:<12} - echo_handler", "HANDLERS");

    let path = request.path.as_str();
    let dynamic_path = &path["/echo/".len()..];
    Response::ok(dynamic_path, ContentType::TextPlain)
}

fn home_handler() -> Response {
    println!("{:<12} - home_handler", "HANDLERS");

    Response::ok("", ContentType::TextPlain)
}

async fn file_path_handler(request: Request) -> Result<Response> {
    println!("{:<12} - file_path_handler", "HANDLERS");

    let path = request.path.as_str();
    let directory_path = config().DIRECTORY.clone();

    println!(
        "{:<12} - file_path_handler - Directory: {:?}",
        "HANDLERS", directory_path
    );

    let filename = &path["/files/".len()..];

    println!(
        "{:<12} - file_path_handler - File Name: {}",
        "HANDLERS", filename
    );

    let full_path = directory_path.as_ref().map_or_else(
        || format!("./{}", filename),
        |dir| format!("{}/{}", dir, filename),
    );

    println!(
        "{:<12} - file_path_handler - Full Path: {}",
        "HANDLERS", &full_path
    );

    Ok(match request.method {
        Method::Get => file_get_handler(&full_path).await?,
        Method::Post => file_post_handler(&full_path, &request.body).await?,
        _ => Response::not_found(),
    })
}

async fn file_post_handler(full_path: &str, file_content: &str) -> Result<Response> {
    println!("{:<12} - file_post_handler", "HANDLERS");

    write_file(full_path, file_content).await?;

    Ok(Response::created(ContentType::OctetStream))
}

async fn file_get_handler(full_path: &str) -> Result<Response> {
    println!("{:<12} - file_get_handler", "HANDLERS");

    let response = read_file_content(full_path)
        .await
        .map_or(Response::not_found(), |content| {
            Response::ok(&content, ContentType::OctetStream)
        });

    Ok(response)
}
