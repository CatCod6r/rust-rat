mod connector;

use connector::Connector;
pub const SERVER: &str = "0.0.0.0:4000";
const SOCKET_ADDRESS: &str = "ws://localhost:4000";

#[tokio::main]
async fn main() {
    let mut connector = Connector::new(SOCKET_ADDRESS);
    //connector.subscribe_for_updates();
    connector.search_for_c2().await;
}
