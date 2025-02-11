mod file_reciever;
mod screenshot_sender;

use std::{net::TcpListener, thread::spawn};
use tungstenite::handshake::client::Response;
use tungstenite::{connect, Message};

pub struct Connector<'a> {
    //Client address recieves data from main server
    socket_addr_client: &'a str,
    //Server address sends data to main server
    socket_addr_server: &'a str,
}
impl<'a> Connector<'a> {
    pub fn new(address_client: &'a str, address_server: &'a str) -> Self {
        Connector {
            socket_addr_client: address_client,
            socket_addr_server: address_server,
        }
    }
    fn debug(response: Response) {
        println!("Connected to the server");
        println!("Response HTTP code: {}", response.status());
        println!("Response contains the following headers:");
        for (header, _value) in response.headers() {
            println!("* {header}");
        }
    }
    pub fn send_data(&self, data: &str) {
        let (mut socket, response) = connect(self.socket_addr_server).expect("Can't connect");
        //debug;
        // Self::debug(response);
        socket.send(Message::Text(data.into())).unwrap();
        let _ = socket.close(None);
    }

    pub fn subscribe_for_updates(&self) {
        let server = TcpListener::bind("127.0.0.1:3012").unwrap();

        for stream in server.incoming() {
            let socket_addr_client = self.socket_addr_client.to_string();
            let socket_addr_server = self.socket_addr_server.to_string();
            spawn(move || {
                use tungstenite::accept;

                let mut websocket = accept(stream.unwrap()).unwrap();
                let mut writing = false;
                let mut buffer: Vec<u8> = Vec::new();
                loop {
                    let msg = websocket.read().unwrap();
                    if writing {
                        file_reciever::recieve_file(msg.clone(), &mut buffer);
                    }
                    match msg.to_string().as_str() {
                        "file_transfer_start" => writing = true,
                        "picture_request" => screenshot_sender::make_screenshot(
                            &socket_addr_client,
                            &socket_addr_server,
                        ),
                        _ => println!("{}", msg),
                    }
                }
            });
        }
    }
}
