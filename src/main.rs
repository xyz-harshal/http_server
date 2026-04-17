mod request;
mod response;

use request::HttpRequest;
use response::{HttpResponse, StatusCode};
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server is Listening to port 8080...");

    loop{
        let (mut socket, addr) = listener.accept().await?;
        println!("New Connection from {}", addr);

        tokio::spawn(async move {
            let mut buffer = vec![0; 2048];

            loop{
                buffer.fill(0);

                let bytes_read = match socket.read(&mut buffer).await {
                    Ok(0) => break,
                    Ok(size) => size,
                    Err(e) => {
                        eprintln!("Error Reading: {}", e);
                        break;
                    }
                };

                let raw_req = String::from_utf8_lossy(&buffer[..bytes_read]);

                let response = match HttpRequest::parse(&raw_req) {
                    Ok(req) => {
                        println!("{} {} {}", req.method, req.path, req.version);

                        if !req.headers.is_empty() {
                            println!(" Headers: {:?}", req.headers);
                        }

                        if let Some(body) = &req.body {
                            if !body.is_empty() {
                                println!(" Body: {}", body);
                            }
                        }

                        handle_route(&req)
                    }
                    Err(e) => {
                        eprintln!("Parse Error: {}", e);
                        HttpResponse::new(StatusCode::BadRequest, "Invalid Request")
                            .header("Content-Type", "text/plain")
                    }
                };

                let res_str = response.build();
                if let Err(e) = socket.write_all(res_str.as_bytes()).await {
                    eprintln!("Failed to write: {}", e);
                }
            }
        });
    }
}

fn handle_route(req: &HttpRequest) -> HttpResponse {
    //Basically will match request from the browser and will respond to it accordingly
    match (req.method.as_str(), req.path.as_str()) {
        ("GET", "/") => HttpResponse::new(StatusCode::Ok, "<h1>Home</h1><a href='/secret'>Secret</a>")
            .header("Content-Type", "text/html"),

        ("GET", "/secret") => HttpResponse::new(StatusCode::Ok, "<h1>🔐 Secret Page</h1>")
            .header("Content-Type", "text/html"),

        ("GET", "/json") => {
            let json_body = r#"{"message": "Hello from Rust!", "status": "ok"}"#;
            HttpResponse::new(StatusCode::Ok, json_body)
                .header("Content-Type", "application/json")
        }

        ("POST", "/echo") => {
            let body = req.body.as_deref().unwrap_or("No body received");
            HttpResponse::new(StatusCode::Ok, body)
                .header("Content-Type", "text/plain")
        }

        ("GET", "/headers") => {
            // Show all received headers — teaches iterator + collect
            let headers_html: String = req
                .headers
                .iter()
                .map(|(k, v)| format!("<li><b>{}</b>: {}</li>", k, v))
                .collect::<Vec<_>>()
                .join("");

            let html = format!("<h1>Your Headers</h1><ul>{}</ul>", headers_html);
            HttpResponse::new(StatusCode::Ok, html)
                .header("Content-Type", "text/html")
        }

        _ => HttpResponse::new(StatusCode::NotFound, "<h1>404 - Not Found</h1>")
            .header("Content-Type", "text/html"),
    }
}
