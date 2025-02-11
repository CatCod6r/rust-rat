mod connector_server;

use std::env;

fn main() {
    let connector = connector_server::ConnectorServer {
        socket_addres: "ws://localhost:3012/socket",
    };
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "sendfile" => connector.send_file(&args[2]),
        _ => println!("No instructions given"),
    }
}
