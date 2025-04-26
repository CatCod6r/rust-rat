use crate::Connector;
use screenshots::{image::EncodableLayout, Screen};

pub fn make_screenshot(socket_adr_server: &str) {
    let screens = Screen::all().unwrap();
    let connector = Connector::new(socket_adr_server);
    for screen in screens {
        let image = screen.capture().unwrap();
        image
            .save(format!("target/{}.png", screen.display_info.id))
            .unwrap();
        connector.send_data("image_transfer_start");
        for byte in image.as_bytes() {
            connector.send_data(format!("{}", byte).as_str());
        }
        connector.send_data("image_transfer_stop");
    }
}
