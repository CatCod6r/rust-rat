use std::net::{IpAddr, SocketAddr};

use futures_util::stream::{SplitSink, SplitStream};
use tokio_tungstenite::WebSocketStream;
use tungstenite::Message;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Instance {
    ip: IpAddr,
    write: SplitSink<WebSocketStream<tokio::net::TcpStream>, Message>,
    read: SplitStream<WebSocketStream<tokio::net::TcpStream>>,
    hostname: String,
    uuid: String,
    public_key: String,
    private_key: String,
}
impl Instance {
    pub fn new(
        addr: SocketAddr,
        write: SplitSink<WebSocketStream<tokio::net::TcpStream>, Message>,
        read: SplitStream<WebSocketStream<tokio::net::TcpStream>>,
        hostname: String,
    ) -> Instance {
        let (public_key, private_key) = generate_keys();
        Instance {
            ip: addr.ip(),
            write,
            read,
            hostname,
            uuid: create_uuid(),
            public_key,
            private_key,
        }
    }
    pub fn init_old(
        addr: SocketAddr,
        write: SplitSink<WebSocketStream<tokio::net::TcpStream>, Message>,
        read: SplitStream<WebSocketStream<tokio::net::TcpStream>>,
        hostname: String,
        uuid: String,
        public_key: String,
        private_key: String,
    ) -> Instance {
        Instance {
            ip: addr.ip(),
            write,
            read,
            hostname,
            uuid,
            public_key,
            private_key,
        }
    }
}
fn create_uuid() -> String {
    "".to_string()
}
fn generate_keys() -> (String, String) {
    ("".to_string(), "".to_string())
}
