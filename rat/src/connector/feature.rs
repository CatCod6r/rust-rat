use super::Connector;

pub mod file_reciever;
pub mod screenshot_sender;
pub mod update;

pub trait Feature {
    fn get_command(&self) -> String;
    async fn run(&self, connector: &mut Connector) -> Result;
}
pub enum Result {
    SUCCESFUL,
    FAILED,
}
