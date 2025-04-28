use serde_json::{json, Value};
use tokio::{
    fs::{self, File, OpenOptions},
    io::{AsyncReadExt, AsyncWriteExt},
};

use crate::USERS_DIRECTORY;

pub struct JsonParser {
    ip: String,
    hostname: String,
    path: String,
}

impl JsonParser {
    pub async fn new(ip: String, hostname: String) -> JsonParser {
        //create a path if not created yet
        let path = format!("{}users.json", USERS_DIRECTORY);
        fs::create_dir_all(USERS_DIRECTORY).await.unwrap();
        let _ = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&path)
            .await
            .unwrap();

        JsonParser { ip, hostname, path }
    }

    pub async fn contains_in_bd(&self) -> Option<String> {
        let mut file = File::open(&self.path).await.unwrap();

        let mut file_content = "".to_string();
        file.read_to_string(&mut file_content);

        let json_data: Value = serde_json::from_str(&file_content).unwrap();

        for data in json_data.as_object().unwrap() {
            if data.0.starts_with("name") {
                if data.1["ip"].as_str().unwrap() == self.ip
                    && data.1["hostname"].as_str().unwrap() == self.hostname
                {
                    return Some(data.0.to_owned());
                } else {
                    return None;
                }
            }
        }
        None
    }

    pub async fn get_keys(&self) -> Option<(String, String)> {
        let mut file = File::open(&self.path).await.unwrap();

        let mut file_content = "".to_string();
        file.read_to_string(&mut file_content);

        let json_data: Value = serde_json::from_str(&file_content).unwrap();
        for data in json_data.as_object().unwrap() {
            match self.contains_in_bd().await {
                Some(name) => {
                    if data.0.to_owned() == name {
                        return Some((
                            data.1["public_key"].to_string(),
                            data.1["private_key"].to_string(),
                        ));
                    }
                }
                None => {
                    return None;
                }
            }
        }
        return None;
    }

    pub async fn save_to_json(&self) -> String {
        let mut file = File::open(&self.path).await.unwrap();

        let mut file_content = "".to_string();
        file.read_to_string(&mut file_content);

        let mut json_data: Value = serde_json::from_str(&file_content).unwrap();

        //what next users name should be
        let next_index = match json_data.as_object() {
            Some(obj) => obj.keys().filter(|key| key.starts_with("name")).count() + 1,
            None => 1,
        };

        let new_name = format!("name{}", next_index);

        // Add the new element to the JSON
        json_data[&new_name] = json!({
            "ip": &self.ip,
            "hostname": &self.hostname,
            "public_key": "",
            "private_key": "",
            "folder": &new_name,
        });

        // Serialize the JSON data back to a string
        let updated_content = serde_json::to_string_pretty(&json_data).unwrap();

        // Write the updated JSON back to the file

        file.write_all(updated_content.as_bytes()).await.unwrap();

        new_name
    }
}
