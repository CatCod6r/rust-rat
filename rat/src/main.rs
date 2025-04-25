mod connector;

use connector::Connector;
const SOCKET_ADDRESS: &str = "ws://localhost:3012/socket";

fn main() {
    let connector = Connector::new(SOCKET_ADDRESS, SOCKET_ADDRESS);
    connector.subscribe_for_updates();
}
