use tokio::net::TcpListener;
use tokio::io::{AsyncWriteExt, AsyncReadExt};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server is Listening on port 8080...");

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {

            let mut buffer = [0; 1024];
            loop{
                let bytes_read = match socket.read(&mut buffer).await {
                    Ok(0) => break,
                    Ok(size) => size,
                    Err(e) => {
                        eprintln!("Error reading from socket {}", e);
                        break;
                    }
                };

                let request = String::from_utf8_lossy(&buffer[..bytes_read]);
                println!("Request recieved:\n {}", request);

                let response = "HTTP/1.1 200 OK\r\n\
                Content-Type: text/html\r\n\
                Content-Length: 30\r\n\
                Connection: keep-alive\r\n\
                \r\n\
                <h1>Hello from Arch Linux!</h1>";

                if let Err(e) = socket.write_all(response.as_bytes()).await {
                    eprintln!("Failed to send response {}", e);
                    break;
                }
                buffer = [0; 1024];
            }
        });
    }
}
