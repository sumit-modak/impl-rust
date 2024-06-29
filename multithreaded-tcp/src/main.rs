use multithreaded_tcp::ThreadPool;
use std::{
    fs,
    io::prelude::*,
    net::{TcpListener, TcpStream},
};

fn main() {
    // listens at the given port (here 7878)
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        if let Ok(stream) = stream {
            pool.execute(|| {
                handle_connection(stream);
            });
        } else {
            println!("Connection failed.")
        }
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let (status, filename) = if buffer.starts_with("GET /".as_bytes()) {
        ("HTTP/1.1 200 OK", "views/index.html")
    } else if buffer.starts_with("GET /sleep".as_bytes()) {
        ("HTTP/1.1 200 OK", "views/index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "views/404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status,
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();

    stream.flush().unwrap();
}
