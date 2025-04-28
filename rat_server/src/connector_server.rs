use std::{cell::RefCell, net::SocketAddr, rc::Rc};

use futures_util::{
    stream::{SplitSink, SplitStream},
    StreamExt,
};
use instance::Instance;
use json_parser::JsonParser;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::WebSocketStream;
use tungstenite::Message;

mod instance;
mod json_parser;

pub struct ConnectorServer {
    pub socket_address: String,
    pub instances: Rc<RefCell<Vec<Instance>>>,
}
impl ConnectorServer {
    pub fn new(socket_address: String) -> ConnectorServer {
        ConnectorServer {
            socket_address,
            instances: Rc::new(RefCell::new(Vec::new())),
        }
    }
    pub async fn run(&self) {
        // Create the event loop and TCP listener we'll accept connections on.
        let try_socket = TcpListener::bind(&self.socket_address).await;
        let listener = try_socket.expect("Failed to bind");
        println!("Listening on: {}", &self.socket_address);

        while let Ok((stream, _)) = listener.accept().await {
            let _ = &self.accept_connection(stream).await;
        }
    }
    pub async fn accept_connection(&self, stream: TcpStream) {
        let addr = stream
            .peer_addr()
            .expect("connected streams should have a peer address");
        let ws_stream = tokio_tungstenite::accept_async(stream)
            .await
            .expect("Error during the websocket handshake occurred");
        println!("New WebSocket connection: {}", addr);

        let (write, mut read) = ws_stream.split();
        if let Some(message) = read.next().await {
            let message_string = message.unwrap().to_string();
            println!("{}", message_string);

            //match our message type and then call creation of instance
            if message_string.contains("ping") {
                //message example: ping|hostname
                let vec: Vec<&str> = message_string.split("|").collect();
                let hostname = vec[1];
                let _ = &self
                    .create_instance(addr, hostname.to_string(), write, read)
                    .await;
            }
        }
    }
    pub async fn create_instance(
        &self,
        addr: SocketAddr,
        hostname: String,
        write: SplitSink<WebSocketStream<tokio::net::TcpStream>, Message>,
        read: SplitStream<WebSocketStream<tokio::net::TcpStream>>,
    ) {
        let json_parser = JsonParser::new(addr.ip().to_string(), hostname.clone());
        if let Some(path) = json_parser.await.contains_in_bd().await {
            if let Some((public_key, private_key)) = json_parser.await.get_keys() {}
            let _ = &self.instances.borrow_mut().push(Instance::init_old(
                addr,
                write,
                read,
                hostname,
                public_key,
                private_key,
                path,
            ));
        } else {
            let path = json_parser.await.save_to_json();
            let instance = Instance::new(addr, write, read, hostname, path);
            let _ = &self.instances.borrow_mut().push(instance);
            //save it in the json
        }
        //Create uuid on the spot ig, same for public/private _keys
    }
}
