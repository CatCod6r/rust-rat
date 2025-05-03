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
                    println!("Current users:");
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
                    let chosen_number: u32 = loop {
                        let input = self.handle_user_input().await.trim().to_string();
                        match input.parse() {
                            Ok(num) => break num,
                            Err(_) => println!("Invalid input. Please enter a valid number."),
                        }
                    };
                    let chosen_instance = &mut instances[(chosen_number - 1) as usize];
                    println!("What command do you choose?");
                    //rust didnt like the prev version :<
                    for (index, feature) in FEATURES.iter().enumerate() {
                        println!("[{}] {}", index + 1, feature);
                    }

                    let chosen_command: u32 = loop {
                        let input = self.handle_user_input().await.trim().to_string();
                        match input.parse() {
                            Ok(num) => break num,
                            Err(_) => println!("Invalid input. Please enter a valid number."),
                        }
                    };
                    //im pretty sure ill need smth more than that
                    chosen_instance
                        .send_encrypted_message(
                            FEATURES[(chosen_command - 1) as usize].to_string().as_str(),
                        )
                        .await;
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
