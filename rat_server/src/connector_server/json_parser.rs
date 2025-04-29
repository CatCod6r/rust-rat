use serde_json::{json, Map, Value};
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
    pub fn new(ip: String, hostname: String) -> JsonParser {
        let path = format!("{}users.json", USERS_DIRECTORY);
        JsonParser { ip, hostname, path }
    }

    pub async fn contains_in_bd(&self) -> Option<String> {
        for data in &self.get_json_data_as_object().await {
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
        for data in &self.get_json_data_as_object().await {
            match self.contains_in_bd().await {
                Some(name) => {
                    if data.0 == &name {
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
        None
    }

    pub async fn save_to_json(&self) -> String {
        let mut file = File::open(&self.path).await.unwrap();

        let mut file_content = "".to_string();
        let _ = file.read_to_string(&mut file_content).await;

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
    async fn get_json_data_as_object(&self) -> Map<String, Value> {
        let mut file = match File::open(&self.path).await {
            Ok(file) => file,
            Err(error) => panic!("Problem opening the file: {error:?}"),
        };
        let mut file_content = "".to_string();
        if let Err(error) = file.read_to_string(&mut file_content).await {
            println!("Problem reading file to string {error:?}");
        }
        let json_data: Value = serde_json::from_str(&file_content).unwrap();
        let data = json_data.as_object().unwrap();
        data.clone()
    }
}
