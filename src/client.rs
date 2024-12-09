use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};
use std::io::{self, Write};

#[tokio::main]
async fn main() {
    let stream = TcpStream::connect("127.0.0.1:4999").await.unwrap();
    let (reader, writer) = stream.into_split();

    // task for receiving messages
    let mut reader = BufReader::new(reader).lines();
    let receive_task = tokio::spawn(async move {
        // this loop continues until the connection closes 
      while let Some(response) = reader.next_line().await.unwrap() {
        print!("\r");  
        println!("{response}");  
        print!("> ");  
        io::stdout().flush().unwrap();
      }
    });

    // task for sending messages
    let mut writer = writer;
    let send_task = tokio::spawn(async move {
        loop {
            print!("> ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            writer.write_all(input.trim().as_bytes()).await.unwrap();
            writer.write_all(b"\n").await.unwrap();   


            // we don't want to see what we entered
            // so ove cursor up one line and clear it
            print!("\x1B[1A\x1B[2K");
            io::stdout().flush().unwrap();
        }
    });

    // wait for either task to finish (like if server disconnects)
    tokio::select! {
        _ = receive_task => println!("Receive task ended"),
        _ = send_task => println!("Send task ended"),
    }
}