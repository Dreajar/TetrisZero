mod board;
mod game;
mod player;
mod server3;
mod tetromino;

use crate::server3::server3::start_game_server;

#[tokio::main] // This attribute macro automatically sets up the Tokio runtime
async fn main() {
    println!("Launching server!");

    // Call the start_game_server function with the required arguments and await its result
    if let Err(e) = start_game_server("127.0.0.1:7878", 10).await {
        eprintln!("Failed to start game server: {}", e);
    }
}