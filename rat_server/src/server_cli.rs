use std::{fmt::format, num::ParseFloatError, time::Duration};

use crate::connector_server::ConnectorServer;

pub struct ServerCli<'a> {
    connector: &'a ConnectorServer,
    messages: Vec<String>,
}
impl ServerCli<'_> {
    pub fn new(connector: &ConnectorServer) -> ServerCli {
        ServerCli {
            connector,
            messages: Vec::new(),
        }
    }
    pub async fn start_cli(&mut self) {
        let mut last_ip = String::new();
        loop {
            let instances = self.connector.get_istances().await.take();
            if let Some(last_instance) = instances.last() {
                let current_ip = last_instance.get_ip().to_string();
                //Checking for changes
                if current_ip != last_ip {
                    for instance in &instances {
                        self.messages.push(format!(
                            "Path:{} IP:{} Hostname:{}",
                            instance.get_path(),
                            instance.get_ip(),
                            instance.get_hostname()
                        ));
                    }

                    //make a logic for interacting with instances

                    //stdin(path);
                    //let chosen_user = instance.get_by_path();
                    //chosen_user.send_file();

                    //Updating last_ip to check for changes
                    last_ip = current_ip;
                }
            }
            let reverse_messages: Vec<String> = self.messages.clone().into_iter().rev().collect();
            for message in reverse_messages {
                println!("{message}");
            }
            self.messages.clear();
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    }
}
