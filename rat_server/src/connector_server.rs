use std::{cell::RefCell, net::SocketAddr, rc::Rc};

use futures_util::{
    stream::{SplitSink, SplitStream},
    StreamExt,
};
use instance::Instance;
use json_parser::JsonParser;
mod hybrid_crypto;
use tokio::{
    fs::{self, OpenOptions},
    net::{TcpListener, TcpStream},
};
use tokio_tungstenite::WebSocketStream;
use tungstenite::Message;

use crate::USERS_DIRECTORY;

pub mod feature;
pub mod instance;
pub mod json_parser;
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
    pub async fn get_istances(&self) -> &Rc<RefCell<Vec<Instance>>> {
        &self.instances
    }
    pub async fn run(&self) {
        // Create the event loop and TCP listener we'll accept connections on.
        let try_socket = TcpListener::bind(&self.socket_address).await;
        let listener = try_socket.expect("Failed to bind");
        println!("Listening on: {}", &self.socket_address);
        //create a path if not created yet
        self.create_path().await;
        while let Ok((stream, _)) = listener.accept().await {
            self.accept_connection(stream).await;
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

        //Start listening for messages
        if let Some(message) = read.next().await {
            let message_string = message.unwrap().to_string();
            println!("{}", message_string);

            //match our message type and then call creation of instance
            if message_string.contains("ping") {
                //pong back to the user
                //message example: ping|hostname
                let vec: Vec<&str> = message_string.split("|").collect();
                let hostname = vec[1];
                self.create_instance(addr, hostname.to_string(), write, read)
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
        if let Some(path) = json_parser.contains_in_bd().await {
            if let Some((public_key, private_key)) = json_parser.get_keys().await {
                self.instances.borrow_mut().push(Instance::init_old(
                    addr,
                    write,
                    read,
                    hostname,
                    Some(public_key),
                    Some(private_key),
                    path,
                ));
            }
        } else {
            //save it in the json
            let path = json_parser.save_to_json().await;
            let mut instance = Instance::new(addr, write, read, hostname, path.clone());
            //Generate keys and send them to json
            json_parser.set_keys(instance.init_keys().await, path).await;
            self.instances.borrow_mut().push(instance);
        }
    }
    pub async fn create_path(&self) {
        let path = format!("{}/users.json", USERS_DIRECTORY);
        fs::create_dir_all(USERS_DIRECTORY).await.unwrap();
        OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&path)
            .await
            .unwrap();
    }
}
