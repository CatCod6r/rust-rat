use std::time::Duration;

use tokio::io::{self, AsyncBufReadExt, BufReader};

use crate::connector_server::{feature::FEATURES, instance::Instance, ConnectorServer};
//Remake this whole shi with RataTUI
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
                    let chosen_number: u32 = get_u32_input().await;
                    let chosen_instance = &mut instances[(chosen_number - 1) as usize];
                    println!("What command do you choose?");
                    //rust didnt like the prev version :<
                    let features = FEATURES.clone();

                    for (index, feature) in features.iter().enumerate() {
                        println!("[{}] {}", index + 1, feature.get_name());
                    }

                    let chosen_command: u32 = get_u32_input().await;
                    let name = features
                        .get((chosen_command - 1) as usize)
                        .unwrap()
                        .get_name();
                    let chosen_feature = features
                        .iter()
                        .find(|feature| feature.get_name() == name)
                        .unwrap();

                    println!("Chosen feature: {}", chosen_feature.get_name());
                    chosen_feature.run(chosen_instance).await;
                    last_ip = current_ip;
                }
            }
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    }
}
pub async fn handle_user_input() -> String {
    let mut input = String::from("");
    let mut stdin = BufReader::new(io::stdin());
    stdin.read_line(&mut input).await.unwrap();
    //remove /n symbol and spaces
    input.trim().to_string()
}
pub async fn get_u32_input() -> u32 {
    loop {
        let input = handle_user_input().await.trim().to_string();
        match input.parse() {
            Ok(num) => return num,
            Err(_) => println!("Invalid input. Please enter a valid number."),
        }
    }
}
