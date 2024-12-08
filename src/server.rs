// server.rs
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:4999").await.unwrap();
    println!("Server listening on 4999...");

    loop {
        let (socket, addr) = listener.accept().await.unwrap();
        println!("New client: {}", addr);
        tokio::spawn(handle_connection(socket));
    }
}

async fn handle_connection(mut stream: TcpStream) {
    let (reader, mut writer) =  stream.split();
    let mut reader = BufReader::new(reader).lines();

    // this loop keeps running as long as the connection is open
    while let Some(line) = reader.next_line().await.unwrap() {
        println!("Got: {}", line);
        // Echo back with a prefix
        writer.write_all(format!("Server received: {}\n", line).as_bytes()).await.unwrap();
    }
}
