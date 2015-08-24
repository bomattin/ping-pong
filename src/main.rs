use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::thread;
use std::collection::vec_deque::VecDeque;

// Create a listener on the Source Engine port.
let listener = TcpListener::bind("0.0.0.0:27010");

// Build a FIFO queue for holding events between ticks.
let events = VecDeque::new();
let active_player_conns = Vec<ToSocketAddrs>;

fn main() {

    println!("Hello, world!");
    for stream_result in listener.incoming() {
        match stream_result {
            Ok(stream) => {
                process_stream(stream);
            }
            Err(e) => {println!("Lost a connection somewhere. {:?}", e);}
        }
    }
    // Server flow:
    // New connection / state check
    // Listen for events from existing connections (ping/pong), queue as they enter
    // Tick (check game logic)

}

fn process_stream(stream: TcpStream) -> UpdatePacket {

}

// Game rules:
// Each player can type either "PING" or "PONG" asychronously (they do not have to wait for a response)
// If a player types the same word as the last word entered, he loses a life
// Each player has 3 lives
