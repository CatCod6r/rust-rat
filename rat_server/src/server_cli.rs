use std::{
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
    collections::HashMap,
    io::BorrowedBuf,
    rc::Rc,
    time::Duration,
};

use tokio::io::{self, AsyncBufReadExt, BufReader};

use crate::connector_server::{instance::Instance, ConnectorServer};

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
                    let features_for_chosen_instance = chosen_instance.get_features();

                    for (index, feature) in features_for_chosen_instance.iter().enumerate() {
                        println!("[{}] {}", index + 1, feature.get_name());
                    }

                    let chosen_command: u32 = loop {
                        let input = self.handle_user_input().await.trim().to_string();
                        match input.parse() {
                            Ok(num) => break num,
                            Err(_) => println!("Invalid input. Please enter a valid number."),
                        }
                    };
                    // Get the feature name immutably first
                    let chosen_feature = chosen_instance.get_feature_by_name(
                        features_for_chosen_instance
                            .get((chosen_command - 1) as usize)
                            .unwrap()
                            .get_name(),
                    );

                    // Now, borrow the chosen_instance mutably
                    chosen_feature.run();
                    //im pretty sure ill need smth more than that
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
