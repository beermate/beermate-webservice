extern crate iron;
extern crate router;
extern crate deque;
extern crate websocket;
extern crate rustc_serialize;

use beermate::Mat;
use beermate::websocket::websocket_server_start;
use iron::prelude::*;
use std::thread;

mod beermate;
mod resources;

fn main() {
    let (worker, stealer) = deque::new::<Mat>();

    thread::spawn(|| {
        websocket_server_start(stealer)
    });
    Iron::new(resources::get_routes(worker)).http("127.0.0.1:3000").unwrap();
}
