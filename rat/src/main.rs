mod connector;

use connector::Connector;
const SOCKET_ADDRESS: &str = "ws://localhost:4000";
const PORT: u16 = 4000;

#[tokio::main]
async fn main() {
    let mut connector = Connector::new(SOCKET_ADDRESS.to_string());
    //connector.subscribe_for_updates();
    connector.kill_process_on_port(PORT);
    connector.search_for_c2().await;
}
