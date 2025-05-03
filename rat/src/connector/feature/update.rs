use futures_util::StreamExt;

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
        let mut hybrid_decryption_arguments: [Vec<u8>; 3] = [Vec::new(), Vec::new(), Vec::new()];
        for index in 0..3 {
            if let Some(message) = connector.read.as_mut().unwrap().next().await {
                //cant put hex decode in the decrypt fn cuz it cant accept
                hybrid_decryption_arguments[index] =
                    hex::decode(message.unwrap().to_string()).unwrap();
            }
        }
        let hybrid_decryption = HybridDecryption::new(
            hybrid_decryption_arguments[0].clone(),
            hybrid_decryption_arguments[1].clone(),
            hybrid_decryption_arguments[2].clone(),
        );
        let decrypted_message = hybrid_decryption.decrypt(connector.private_key.clone());
        Result::FAILED
    }
}
impl Update {
    pub fn new() -> Update {
        Update {
            name: "update".to_string(),
        }
    }
}
