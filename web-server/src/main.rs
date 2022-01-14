use std::{
    fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:5000").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0_u8; 1024];

    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let get = b"Request: GET / HTTP/1.1";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "src/html/index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "src/html/404.html")
    };

    let html = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        html.len(),
        html
    );

    println!("content len = {}", response.len());
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
