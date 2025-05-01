use std::time::Duration;

use connector_server::ConnectorServer;
use tokio::join;

mod connector_server;

pub const USERS_DIRECTORY: &str = "users";

#[tokio::main]
async fn main() {
    let connector = ConnectorServer::new("localhost:4000".to_string());
    join!(connector.run(), start_cli(&connector));
}
async fn start_cli(connector: &ConnectorServer) {
    let mut last_ip = String::new();
    loop {
        let instances = connector.get_istances().await.take();
        if let Some(last_instance) = instances.last() {
            let current_ip = last_instance.get_ip().to_string();
            //Checking for changes
            if current_ip != last_ip {
                for instance in &instances {
                    println!(
                        "Path:{} IP:{} Hostname:{}",
                        instance.get_path(),
                        instance.get_ip(),
                        instance.get_hostname()
                    );
                }
                //make a logic for interacting with instances

                //stdin(path);
                //let chosen_user = instance.get_by_path();
                //chosen_user.send_file();

                //Updating last_ip to check for changes
                last_ip = current_ip;
            }
        }
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}
