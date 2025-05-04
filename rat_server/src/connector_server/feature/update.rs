use crate::connector_server::instance::{self, Instance};

#[derive(Debug)]
pub struct Update {
    name: String,
}

impl Update {
    pub fn new() -> Update {
        Update {
            name: "update".to_string(),
        }
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub async fn run(&self, _instance: &mut Instance) {
        println!("Update running");
    }
}
