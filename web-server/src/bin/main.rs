use std::{
    fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread, time,
};

use web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:5000").unwrap();
    let thread_pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        // handle_single_threaded_connection(stream);
        thread_pool.execute(|| handle_single_threaded_connection(stream));
    }
}

fn handle_single_threaded_connection(mut stream: TcpStream) {
    let mut buffer = [0_u8; 1024];

    stream.read(&mut buffer).unwrap();

    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let get = b"Request: GET / HTTP/1.1";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "src/html/index.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(time::Duration::from_secs(5));
        ("HTTP/1.1 404 NOT FOUND", "src/html/index.html")
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

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
