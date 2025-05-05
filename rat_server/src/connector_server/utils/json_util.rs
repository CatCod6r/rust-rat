use serde_json::{json, Map, Value};
use tokio::{fs::OpenOptions, io::AsyncWriteExt};

use crate::USERS_PATH;

pub async fn contains_in_json(ip: &str, hostname: &str) -> Option<String> {
    let json_data = get_json_data_as_object(&USERS_PATH).await;

    for (key, value) in &json_data {
        if key.starts_with("name") {
            if value["ip"].as_str().unwrap_or("") == ip
                && value["hostname"].as_str().unwrap_or("") == hostname
            {
                return Some(key.clone());
            } else {
                return None;
            }
        }
    }
    None
}

pub async fn save_to_json(ip: &str, hostname: &str) -> String {
    let file_content = match tokio::fs::read_to_string(&USERS_PATH).await {
        Ok(content) if !content.is_empty() => content,
        _ => String::from("{}"),
    };

    let mut json_data: Value = serde_json::from_str(&file_content).unwrap();
    let next_index = match json_data.as_object() {
        Some(obj) => obj.keys().filter(|key| key.starts_with("name")).count() + 1,
        None => 1,
    };
    let new_name = format!("name{}", next_index);

    json_data[&new_name] = json!({
        "ip": ip,
        "hostname": hostname,
        "folder": &new_name,
    });

    let updated_content = serde_json::to_string_pretty(&json_data).unwrap();
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&USERS_PATH)
        .await
        .unwrap();
    file.write_all(updated_content.as_bytes()).await.unwrap();

    new_name
}

pub async fn get_json_data_as_object(path: &str) -> Map<String, Value> {
    let file_content = match tokio::fs::read_to_string(path).await {
        Ok(content) if !content.is_empty() => content,
        _ => String::from("{}"),
    };

    let json_data: Value = serde_json::from_str(&file_content).unwrap();
    json_data.as_object().unwrap().clone()
}
