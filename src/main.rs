pub mod client;
pub mod server;

#[tokio::main]
async fn main() {
    let arg: String = std::env::args().nth(1).expect("no argument provided");

    println!("{arg}");
    if arg == "--server" {
        println!("Running server");
        server::run_server().await;
    } else if arg == "--client" {
    }
}
