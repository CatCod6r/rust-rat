use std::{
    borrow::{Borrow, BorrowMut},
    time::Duration,
};

use connector_server::{instance::Instance, ConnectorServer};

mod connector_server;

pub const USERS_DIRECTORY: &str = "users";

#[tokio::main]
async fn main() {
    let connector = ConnectorServer::new("0.0.0.0:4000".to_string());
    connector.run().await;
    start_cli(connector.get_istances().take()).await;
}
async fn start_cli(instances: Vec<Instance>) {
    let mut last_instances: &Vec<Instance> = &Vec::new();
    loop {
        if instances.last().unwrap().get_ip() != last_instances.last().unwrap().get_ip() {
            for instance in &instances {
                println!("{:?}", instance);
            }
        }
        last_instances = &instances;
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}
