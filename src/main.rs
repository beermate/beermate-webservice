extern crate deque;
extern crate websocket;
extern crate rustc_serialize;

use beermate::websocket::websocket_server_start;
use std::thread;

mod beermate;

fn main() {
    let websocket_thread = thread::spawn(websocket_server_start);

    websocket_thread.join().unwrap();
}
