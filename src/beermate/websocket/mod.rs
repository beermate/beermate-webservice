use beermate::Mat;
use deque::{Stealer};
use deque::Stolen;
use rustc_serialize::json;
use std::thread;
use websocket::{Message, Sender, Server};
use websocket::sender;
use websocket::stream::WebSocketStream;

pub fn websocket_server_start(stealer: Stealer<Mat>) {
    let server = Server::bind("192.168.0.138:2794").unwrap();

    for connection in server {
        let stealer = stealer.clone();

        let request = connection.unwrap().read_request().unwrap();
        request.validate().unwrap();

        let response = request.accept();
        let client = response.send().unwrap();

        let (mut sender, _) = client.split();

        thread::spawn(move || {
            start_connection(&mut sender, stealer);
        });
    }
}

fn start_connection(sender: &mut sender::Sender<WebSocketStream>, stealer: Stealer<Mat>) {
    while true {
        let message = stealer.steal();

        match message {
            Stolen::Empty => {},
            Stolen::Abort => println!("something went wrong during stealing"),
            Stolen::Data(value) => {
                let message = Message::text(json::encode(&value).unwrap());
                sender.send_message(&message).unwrap();
            }
        }
    }
}
