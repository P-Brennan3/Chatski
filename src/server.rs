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
    // tx is the sender side, _rx is receiver we don't need (since each connection makes their own)
    let (tx, _rx) = broadcast::channel(100);

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
        tokio::spawn(handle_connection(socket, tx, rx, addr));
    }
}

// function to handle each client connection
async fn handle_connection(
    stream: TcpStream,
    tx: broadcast::Sender<(String, std::net::SocketAddr)>,  // to send messages
    mut rx: broadcast::Receiver<(String, std::net::SocketAddr)>,  // to receive messages
    addr: std::net::SocketAddr,  // Client's address
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
        while let Ok((msg, from_addr)) = rx.recv().await {
            // Format and send message to client
          writer.write_all(format!("{}: {}\n", from_addr, msg).as_bytes()).await.unwrap();  
        }
    });

    // main loop for this client - read their messages and broadcast them
    // this loop runs indefinitely until:
    //   * the client disconnects (which would make next_line() return None)
    //   * there's an error (which would panic due to the unwrap)
    while let Some(line) = reader.next_line().await.unwrap() {
        println!("Got message from {}: {}", addr, line);
        // send message to broadcast channel along with sender's address 
        // so that other connect clients will receive it (via the task above -> tx.recv())
        tx.send((line, addr)).unwrap();
    }
}