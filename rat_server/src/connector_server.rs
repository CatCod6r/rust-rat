use futures_util::StreamExt;
use tokio::net::{TcpListener, TcpStream};

pub struct ConnectorServer<'a> {
    pub socket_addres: &'a str,
}
impl<'a> ConnectorServer<'a> {
    pub async fn run(&self) {
        let addr = "0.0.0.0:4000".to_string();

        // Create the event loop and TCP listener we'll accept connections on.
        let try_socket = TcpListener::bind(&addr).await;
        let listener = try_socket.expect("Failed to bind");
        println!("Listening on: {}", addr);

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
        println!("{}", message.unwrap().to_string());
    }

    /*
    read.try_filter(|msg| future::ready(msg.is_text() || msg.is_binary()))
        .forward(write)
        .await
        .expect("Failed to forward messages")
    */
}
