use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::time::{sleep, Duration};
use std::net::SocketAddr;

/// Handles an incoming TCP connection.
///
/// # Arguments
///
/// * `stream` - A mutable `TcpStream` representing the incoming TCP connection.
///
/// # Behavior
///
/// The function reads the request line from the stream using a buffered reader.
/// Depending on the request line, it determines the status line and the filename
/// of the HTML file to be served.
///
/// # Errors
///
/// This function will panic if reading from the stream, reading the file, or
/// writing to the stream fails.
async fn handle_connection(mut stream: TcpStream, socket_addr: SocketAddr) {
    let mut buf_reader = BufReader::new(&mut stream);
    let mut request_line = String::new();
    buf_reader.read_line(&mut request_line).await.unwrap();

    println!("Recieved request from {}", socket_addr.to_string());

    let (status_line, filename) = match request_line.trim() {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            sleep(Duration::from_secs(5)).await;
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = tokio::fs::read_to_string(filename).await.unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).await.unwrap();
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();

    loop {
        let (stream, socket_addr) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            handle_connection(stream, socket_addr).await;
        });
    }
}
