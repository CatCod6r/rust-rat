use rsa::{
    pkcs1::{DecodeRsaPrivateKey, DecodeRsaPublicKey},
    RsaPrivateKey, RsaPublicKey,
};
use serde_json::{json, Map, Value};
use tokio::{fs::OpenOptions, io::AsyncWriteExt};

use crate::USERS_DIRECTORY;

pub struct JsonParser {
    ip: String,
    hostname: String,
    path: String,
}

impl JsonParser {
    pub fn new(ip: String, hostname: String) -> JsonParser {
        let path = format!("{}/users.json", USERS_DIRECTORY);
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

    pub async fn save_to_json(&self) -> String {
        let file_content = match tokio::fs::read_to_string(&self.path).await {
            Ok(content) => {
                if content.is_empty() {
                    String::from("{}")
                } else {
                    content
                }
            }
            Err(_) => String::from("{}"),
        };
        let mut json_data: Value = serde_json::from_str(&file_content).unwrap();
        //What next users name should be
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
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.path)
            .await
            .unwrap();
        file.write_all(updated_content.as_bytes()).await.unwrap();
        new_name
    }
    async fn get_json_data_as_object(&self) -> Map<String, Value> {
        let file_content = match tokio::fs::read_to_string(&self.path).await {
            Ok(content) => {
                if content.is_empty() {
                    String::from("{}")
                } else {
                    content
                }
            }
            Err(_) => String::from("{}"),
        };
        let json_data: Value = serde_json::from_str(&file_content).unwrap();
        let data = json_data.as_object().unwrap();
        data.clone()
    }
    pub async fn get_keys_string(&self) -> Option<(String, String)> {
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
    pub async fn get_keys(&self) -> Option<(RsaPublicKey, RsaPrivateKey)> {
        if let Some((public_key, private_key)) = &self.get_keys_string().await {
            Some((
                RsaPublicKey::from_pkcs1_pem(public_key.as_str()).unwrap(),
                RsaPrivateKey::from_pkcs1_pem(private_key.as_str()).unwrap(),
            ))
        } else {
            None
        }
    }
    pub async fn set_keys(&self, (public_key, private_key): (String, String), path: String) {
        let json_data = &mut self.get_json_data_as_object().await;
        if let Some(entry) = json_data.get_mut(path.as_str()) {
            if let Some(obj) = entry.as_object_mut() {
                obj.insert("public_key".to_string(), json!(public_key));
                obj.insert("private_key".to_string(), json!(private_key));
            }
        }
        let updated_content = serde_json::to_string_pretty(&json_data).unwrap();
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.path)
            .await
            .unwrap();
        file.write_all(updated_content.as_bytes()).await.unwrap();
    }
}
