use std::net::{TcpListener, TcpStream, ToSocketAddrs, Ipv4Addr};
use std::thread;
use std::collections::vec_deque::VecDeque;
use std::io::{BufReader,Read};

fn main() {

    // Create a listener on the Source Engine port.
    let listener = TcpListener::bind("0.0.0.0:27010").unwrap();

    // Build a FIFO queue for holding events between ticks.
    // let events = VecDeque::new();
    let active_player_conns: Vec<String> = Vec::new();



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

fn process_stream(stream: TcpStream) {
    let mut reader = BufReader::new(stream);
    let mut result_string: String = String::from("");
    let size = reader.read_to_string(&mut result_string).unwrap();
    println!("Read {} bytes. ", size);
    println!("Message follows: ");
    println!("{}", result_string);
}

// Game rules:
// Each player can type either "PING" or "PONG" asychronously (they do not have to wait for a response)
// If a player types the same word as the last word entered, he loses a life
// Each player has 3 lives
