extern crate rustc_serialize;
extern crate websocket;

use rustc_serialize::json;
use std::thread;
use websocket::{Server, Message, Sender, Receiver};
use websocket::message::Type;

#[derive(RustcEncodable)]
struct Matt {
    id: u32,
    level: f32,
    beer_on_matt: bool
}

fn main() {
    let server = Server::bind("127.0.0.1:2794").unwrap();

    for connection in server {
        thread::spawn(move || {
            let request = connection.unwrap().read_request().unwrap();
            request.validate().unwrap();

            let response = request.accept();
            let client = response.send().unwrap();

            let (mut sender, mut receiver) = client.split();

            for message in receiver.incoming_messages() {
                let message: Message = message.unwrap();

                match message.opcode {
                    Type::Text => {
                        let message = Message::text(json::encode(&Matt{id: 1, level: 0.74, beer_on_matt: false}).unwrap());
                        sender.send_message(&message).unwrap();
                    },
                    _ => {
                        let message = Message::text("Wrong opcode".to_string());
                        sender.send_message(&message).unwrap();
                    }
                }
            }
        });
    }
}
