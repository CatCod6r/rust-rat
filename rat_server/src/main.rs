use connector_server::ConnectorServer;
use server_cli::ServerCli;
use tokio::join;

mod connector_server;
mod server_cli;

pub const USERS_DIRECTORY: &str = "users";

#[tokio::main]
async fn main() {
    let connector = ConnectorServer::new("localhost:4000".to_string());
    //initializing features(yes its silly)
    let server_cli = ServerCli::new(&connector);
    join!(connector.run(), server_cli.start_cli());
}
