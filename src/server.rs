use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
    sync::broadcast,
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:4999").await.unwrap();
    println!("Server listening on 4999...");

    // create broadcast channel - every message sent through this will go to all subscribers
    // tx is the sender side, _ is receiver we don't need (since each connection makes their own)
    let (tx, _) = broadcast::channel(100);

    // main server loop - continuously accept new connections
    loop {
        // wait for new client to connect, socket is their connection, addr is their IP/port
        let (socket, addr) = listener.accept().await.unwrap();
        println!("New client: {}", addr);

        // clone tx so this client can send messages to channel
        let tx = tx.clone();
        // create new receiver so this client can receive messages from channel
        let rx = tx.subscribe();

        // spawn new task to handle this client
        tokio::spawn(handle_connection(socket, tx, rx));
    }
}

// function to handle each client connection
async fn handle_connection(
    stream: TcpStream,
    tx: broadcast::Sender<(String, String)>, // to send messages
    mut rx: broadcast::Receiver<(String, String)>, // to receive messages
) {
    // split connection into read and write parts
    let (reader, writer) = stream.into_split();
    // create buffered reader for reading lines
    let mut reader = BufReader::new(reader).lines();

    // spawn task to handle receiving broadcast messages
    let mut writer = writer;
    tokio::spawn(async move {
        //  this loop runs indefinitely
        // continuously receiving messages from broadcast channel and writing to client stream
        while let Ok((msg, username)) = rx.recv().await {
            // Format and send message to client
            writer
                .write_all(format!("{}: {}\n", username, msg).as_bytes())
                .await
                .unwrap();
        }
    });

    // main loop for this client - read the client's messages and broadcasting them
    // this loop runs indefinitely until:
    //   * the client disconnects (which would make next_line() return None)
    //   * there's an error (which would panic due to the unwrap)
    while let Some(line) = reader.next_line().await.unwrap() {
        if let Some((username, message)) = line.split_once(':') {
            println!("Got message from {}: {}", username, message);
            let _ = tx.send((message.to_string(), username.to_string()));
        }
    }
}
