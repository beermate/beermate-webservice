extern crate iron;
extern crate router;

use iron::prelude::*;

mod resources;

fn main() {
    Iron::new(resources::get_routes()).http("127.0.0.1:3000").unwrap();
}
