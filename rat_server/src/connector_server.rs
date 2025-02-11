use std::fs::File;
use std::io::Read;
use tungstenite::connect;

pub struct ConnectorServer<'a> {
    pub socket_addres: &'a str,
}
impl<'a> ConnectorServer<'a> {
    pub fn send_file(&self, location: &str) {
        let (mut socket, _response) = connect(self.socket_addres).expect("Can't connect");
        let mut file = File::open(location).expect("Cant open the file");
        let mut buffer = Vec::new();

        file.read_to_end(&mut buffer)
            .expect("Cannot read file to buffer");

        //Sends packet type before hand so client know what to accept
        socket
            .send("file_transfer_start".into())
            .expect("Didnt send packet type");

        //Sends file
        for byte in buffer {
            socket
                .send(format!("{:08b}", byte).into())
                .expect("Cant send data");
        }

        //Sends stop packet
        socket
            .send("file_transfer_stop".into())
            .expect("Didnt send packet type");
        socket.close(None).expect("Cant close socket");
    }
}
