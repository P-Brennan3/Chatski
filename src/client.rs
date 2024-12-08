// client.rs
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};
use std::io::{self, Write};

#[tokio::main]
async fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:4999").await.unwrap();
    let (reader, mut writer) = stream.split();
    let mut reader = BufReader::new(reader).lines();

    println!("Connected! Type messages (Ctrl+C to quit)");
    
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        
         // Get input, send it on the SAME connection
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        writer.write_all(input.as_bytes()).await.unwrap();
        writer.write_all(b"\n").await.unwrap();

        if let Some(response) = reader.next_line().await.unwrap() {
            println!("{}", response);
        }
    }
}