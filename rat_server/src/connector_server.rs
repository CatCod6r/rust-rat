use std::{cell::RefCell, net::SocketAddr, rc::Rc};

use futures_util::{
    stream::{SplitSink, SplitStream},
    StreamExt,
};
use instance::Instance;
mod utils;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::WebSocketStream;
use tungstenite::Message;
use utils::{
    file_util::create_file,
    json_util::{contains_in_json, save_to_json},
};

use crate::USERS_PATH;

pub mod feature;
pub mod instance;

pub type WsStream = WebSocketStream<TcpStream>;
pub type WsWrite = SplitSink<WsStream, Message>;
pub type WsRead = SplitStream<WsStream>;

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
        create_file(USERS_PATH).await;
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
        write: WsWrite,
        read: WsRead,
    ) {
        let path = match contains_in_json(addr.ip().to_string().as_str(), hostname.as_str()).await {
            Some(existing_path) => existing_path,
            None => save_to_json(addr.ip().to_string().as_str(), hostname.as_str()).await,
        };

        let mut instance = if contains_in_json(addr.ip().to_string().as_str(), hostname.as_str())
            .await
            .is_some()
        {
            Instance::init_old(addr, write, read, hostname, None, None, path.clone())
        } else {
            Instance::new(addr, write, read, hostname, path.clone())
        };

        instance.init_keys().await;
        self.instances.borrow_mut().push(instance);
    }
}
