use tokio::{
    fs,
    io::{self, AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() -> io::Result<()> {
    // listens at the given port (here 7878)
    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();

    loop {
        let (socket, _) = listener.accept().await?;
        handle_connection(socket).await;
    }
}

async fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await.unwrap();

    let (status, filename) = if buffer.starts_with("GET /".as_bytes()) {
        ("HTTP/1.1 200 OK", "views/index.html")
    } else if buffer.starts_with("GET /sleep".as_bytes()) {
        ("HTTP/1.1 200 OK", "views/index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "views/404.html")
    };

    let contents = fs::read_to_string(filename).await.unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status,
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).await.unwrap();

    stream.flush().await.unwrap();
}
