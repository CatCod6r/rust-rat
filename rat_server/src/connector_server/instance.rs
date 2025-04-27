use std::net::{IpAddr, SocketAddr};

use tokio::net::TcpStream;

pub struct Instance {
    ip: IpAddr,
    stream: TcpStream,
    hostname: String,
    uuid: String,
    public_key: String,
    private_key: String,
}
impl Instance {
    pub fn new(addr: SocketAddr, stream: TcpStream, hostname: String) -> Instance {
        let (public_key, private_key) = generate_keys();
        Instance {
            ip: addr.ip(),
            stream,
            hostname,
            uuid: create_uuid(),
            public_key,
            private_key,
        }
    }
    pub fn init_old(
        addr: SocketAddr,
        stream: TcpStream,
        hostname: String,
        uuid: String,
        public_key: String,
        private_key: String,
    ) -> Instance {
        Instance {
            ip: addr.ip(),
            stream,
            hostname,
            uuid,
            public_key,
            private_key,
        }
    }
}
fn create_uuid() -> String {}
fn generate_keys() -> (String, String) {}
