pub struct JsonParser {
    ip: String,
    hostname: String,
}
impl JsonParser {
    pub fn new(ip: String, hostname: String) -> JsonParser {
        JsonParser { ip, hostname }
    }
    pub fn contains_in_bd(&self) -> bool {
        false
    }
    pub fn get_uuid(&self) -> String {
        "".to_string()
    }
    pub fn get_keys(&self) -> (String, String) {
        ("".to_string(), "".to_string())
    }
}
