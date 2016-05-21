extern crate deque;

use beermate::Mat;
use deque::Stealer;
use deque::Stolen;
use rustc_serialize::json;
use std::thread;
use std::time::Duration;
use websocket::{Message, Sender, Server};
use websocket::sender;
use websocket::stream::WebSocketStream;

pub fn websocket_server_start() {
    let server = Server::bind("127.0.0.1:2794").unwrap();

    let (worker, stealer) = deque::new::<Mat>();

    // TODO remove when we get some real data
    thread::spawn(move || {
        while true {
            println!("pushed value");
            let mat = Mat{id: 1, level: 0.74, beer_on_mat: true};
            worker.push(mat);
            thread::sleep(Duration::new(10, 0));
        }
    });

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
            Stolen::Empty => println!("no data"),
            Stolen::Abort => println!("something went wrong during stealing"),
            Stolen::Data(value) => {
                let message = Message::text(json::encode(&value).unwrap());
                sender.send_message(&message).unwrap();
            }
        }
    }
}
