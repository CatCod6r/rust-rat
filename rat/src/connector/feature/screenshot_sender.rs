use crate::connector::Connector;

use super::Result;

#[derive(Clone, Debug)]
pub struct Screenshot {
    name: String,
}
impl Screenshot {
    pub fn new() -> Screenshot {
        Screenshot {
            name: "update".to_string(),
        }
    }
    pub fn get_command(&self) -> String {
        "update".to_string()
    }
    pub async fn run(&self, connector: &mut Connector) -> Result {}
}

