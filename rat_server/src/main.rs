use std::time::Duration;

use connector_server::{instance::Instance, ConnectorServer};

mod connector_server;

pub const USERS_DIRECTORY: &str = "users";

#[tokio::main]
async fn main() {
    let connector = ConnectorServer::new("0.0.0.0:4000".to_string());
    connector.run().await;
    start_cli(connector).await;
}
async fn start_cli(connector: ConnectorServer) {
    let mut last_ip = String::new();
    loop {
        let instances = connector.get_istances().await.take();
        if let Some(last_instance) = instances.last() {
            let current_ip = last_instance.get_ip().to_string();
            if current_ip != last_ip {
                for instance in &instances {
                    println!("{:?}", instance);
                }
                last_ip = current_ip;
            }
        }
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}
