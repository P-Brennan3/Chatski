use std::fs;
use std::io::{self, Write};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

#[tokio::main]
async fn main() {
    let username = get_username();

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
            // wait for user input, read_line keeps reading until it hits \n (user hitting the enter key)
            io::stdin().read_line(&mut input).unwrap();
            writer
                .write_all(format!("{}:{}\n", username, input.trim()).as_bytes())
                .await
                .unwrap();
            writer.write_all(b"\n").await.unwrap();

            // we don't want to see what we entered
            // so move cursor up one line and clear it
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

fn get_username() -> String {
    match fs::read_to_string("chatski.config") {
        Ok(contents) => {
            for line in contents.lines() {
                if let Some(username) = line.strip_prefix("username = ") {
                    return username.trim_matches('"').to_string();
                }
            }
            "Anonymous".to_string()
        }
        Err(_) => "Anonymous".to_string(),
    }
}
