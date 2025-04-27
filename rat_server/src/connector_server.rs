use std::{thread, time::Duration};

use futures_util::{stream::SplitSink, SinkExt, StreamExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::WebSocketStream;
use tungstenite::Message;
mod instance;

pub struct ConnectorServer {
    pub socket_address: String,
}
impl ConnectorServer {
    pub fn new(socket_address: String) -> ConnectorServer {
        ConnectorServer { socket_address }
    }
    pub async fn run(&self) {
        // Create the event loop and TCP listener we'll accept connections on.
        let try_socket = TcpListener::bind(&self.socket_address).await;
        let listener = try_socket.expect("Failed to bind");
        println!("Listening on: {}", &self.socket_address);

        while let Ok((stream, _)) = listener.accept().await {
            tokio::spawn(accept_connection(stream));
        }
    }
}
pub async fn accept_connection(stream: TcpStream) {
    let addr = stream
        .peer_addr()
        .expect("connected streams should have a peer address");
    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("New WebSocket connection: {}", addr);
    let (write, mut read) = ws_stream.split();
    if let Some(message) = read.next().await {
        println!("{}", message.unwrap());
    }
    //send_message_loop(write).await;
    /*
    read.try_filter(|msg| future::ready(msg.is_text() || msg.is_binary()))
        .forward(write)
        .await
        .expect("Failed to forward messages")
    */
}
/*
pub async fn send_message_loop(
    mut write: SplitSink<WebSocketStream<tokio::net::TcpStream>, Message>,
) {
    loop {
        println!("trying to send");
        if let Err(err) = write.send(Message::from("haiiiiiiii")).await {
            println!("{err}");
        }
        thread::sleep(Duration::from_secs(3));
    }
}
*/
