use connector_server::ConnectorServer;

mod connector_server;

pub const USERS_DIRECTORY: &str = "users";

#[tokio::main]
async fn main() {
    let connector = ConnectorServer::new("0.0.0.0:4000".to_string());
    //for some reason its not the socket but okay ig
    //"ws://0.0.0.0:4000/socket

    //let args: Vec<String> = env::args().collect();
    //
    connector.run().await;
    //tf is this :sob:
    //match args[1].as_str() {
    //"sendfile" => connector.send_file(&args[2]),
    //    _ => println!("No instructions given"),
    //}
}
