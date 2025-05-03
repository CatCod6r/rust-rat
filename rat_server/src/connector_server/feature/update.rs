use crate::connector_server::instance::{self, Instance};

use super::Feature;

pub struct Update {
    name: String,
}
impl Feature for Update {
    fn get_name(&self) -> String {
        self.name.clone()
    }
    async fn run(&self, instance: Instance) {}
}
impl Update {
    pub fn new() -> Update {
        Update {
            name: "update".to_string(),
        }
    }
}
