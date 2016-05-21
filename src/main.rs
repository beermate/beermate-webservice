extern crate iron;
extern crate router;
extern crate deque;
extern crate websocket;
extern crate rustc_serialize;

use iron::prelude::*;
use std::thread;
use beermate::Mat;
use beermate::websocket::websocket_server_start;

mod beermate;
mod resources;

fn main() {
    let (worker, stealer) = deque::new::<Mat>();

    thread::spawn(|| {
        websocket_server_start(stealer)
    });

    Iron::new(resources::get_routes(worker)).http("0.0.0.0:3000").unwrap();
}
