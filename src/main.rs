use tokio::net::TcpListener;
use tokio::io::{AsyncWriteExt, AsyncReadExt};

struct HttpReq{
    method : String,
    path : String,
}

impl HttpReq {
    fn parse(raw_req: &str) -> Option<Self> {
        let first_line = raw_req.lines().next()?;
        let mut parts = first_line.split_whitespace();
        let method = parts.next()?.to_string();
        let path = parts.next()?.to_string();
        Some(Self {method, path})
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server is Listening on port 8080...");

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {

            let mut buffer = vec![0; 2048];
            loop{
                buffer.fill(0);

                let bytes_read = match socket.read(&mut buffer).await {
                    Ok(0) => break, //Connection closed by the client
                    Ok(size) => size,
                    Err(e) => {
                        eprintln!("Error reading from socket {}", e);
                        break;
                    }
                };

                let raw_req = String::from_utf8_lossy(&buffer[..bytes_read]);

                let res = match HttpReq::parse(&raw_req) {
                    Some(req) => {
                        println!("Recieved: {} {}", req.method, req.path);
                        //Based on request paths it will dynamically render the response 
                        let body = match req.path.as_str() {
                            "/" => "<h1>Home page</h1>",
                            "/secret" => "<h1>Secret Page</h1>",
                            _  => "<h1>404 not found</h1>",
                        };
                        format!(
                            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{}",
                            body.len(), body
                        )
                    }
                    None => "HTTP/1.1 400 Bad Request\r\nContent-Length: 17\r\n\r\nInvalid Request".to_string()
                };

                if let Err(e) = socket.write_all(res.as_bytes()).await {
                    eprintln!("Failed to send response {}", e);
                }
            }
        });
    }
}
