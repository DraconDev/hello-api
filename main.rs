use std::net::TcpListener;
use std::io::{Read, Write};

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8080")?;
    println!("hello-api listening on :8080");
    
    for stream in listener.incoming() {
        let mut stream = stream?;
        let mut buf = [0; 1024];
        let n = stream.read(&mut buf)?;
        let request = String::from_utf8_lossy(&buf[..n]);
        
        let response = if request.contains("GET /health") {
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{\"status\":\"ok\"}"
        } else {
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nHello from pully-managed service!"
        };
        
        stream.write_all(response.as_bytes())?;
    }
    Ok("
}
