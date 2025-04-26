mod connector_server;

use std::env;

#[tokio::main]
async fn main() {
    let connector = connector_server::ConnectorServer {
        socket_addres: "ws://0.0.0.0:4000/socket",
    };

    //let args: Vec<String> = env::args().collect();
    connector.run().await;
    //tf is this :sob:
    //match args[1].as_str() {
    //"sendfile" => connector.send_file(&args[2]),
    //    _ => println!("No instructions given"),
    //}
}
