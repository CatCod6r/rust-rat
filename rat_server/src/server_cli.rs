use std::{collections::HashMap, time::Duration};

use tokio::io::{self, AsyncBufReadExt, BufReader};

use crate::connector_server::{
    instance::{Instance, FEATURES},
    ConnectorServer,
};

pub struct ServerCli<'a> {
    connector: &'a ConnectorServer,
}
impl ServerCli<'_> {
    pub fn new(connector: &ConnectorServer) -> ServerCli {
        ServerCli { connector }
    }
    pub async fn start_cli(&self) {
        let mut last_ip = String::new();

        loop {
            let instances = &mut self.connector.get_istances().await.take();
            if let Some(last_instance) = instances.last() {
                let current_ip = last_instance.get_ip().to_string();
                //Checking for changes
                if current_ip != last_ip {
                    for index in 0..instances.len() {
                        let instance: &Instance = instances.get(index).unwrap();
                        println!(
                            "[{}] Path:{} IP:{} Hostname:{}",
                            index + 1,
                            instance.get_path(),
                            instance.get_ip(),
                            instance.get_hostname()
                        );
                    }
                    println!("Input the number of user:");
                    let chosen_number = self.handle_user_input().await.parse::<usize>().unwrap();
                    let chosen_instance = &mut instances[chosen_number - 1];
                    println!("What command do you choose?");
                    for index in 0..FEATURES.len() {
                        print!("[{}] {}", index + 1, FEATURES[index]);
                    }
                    let chosen_command = self.handle_user_input().await.parse::<usize>().unwrap();
                    if chosen_instance.is_keys_init() {
                        chosen_instance
                            .send_chosen_command(FEATURES[chosen_command - 1].to_string())
                            .await;
                    }
                    last_ip = current_ip;
                }
            }
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    }

    pub async fn handle_user_input(&self) -> String {
        let mut input = String::from("");
        let mut stdin = BufReader::new(io::stdin());
        stdin.read_line(&mut input).await.unwrap();
        input
    }
}
