extern crate iron;
extern crate router;
extern crate deque;
extern crate websocket;
extern crate rustc_serialize;

use iron::prelude::*;
use beermate::websocket::websocket_server_start;
use std::thread;

mod beermate;
mod resources;

fn main() {
    thread::spawn(websocket_server_start);
    Iron::new(resources::get_routes()).http("127.0.0.1:3000").unwrap();
}
