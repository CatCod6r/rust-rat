use once_cell::unsync::Lazy;
use screenshot_sender::Screenshot;
use update::Update;

use super::Connector;

pub mod screenshot_sender;
pub mod update;

pub static FEATURES: Lazy<Vec<FeatureEnum>> = Lazy::new(|| {
    vec![
        FeatureEnum::Update(Update::new()),
        FeatureEnum::Screenshot(Screenshot::new()),
    ]
});

#[derive(Debug, Clone)]
pub enum FeatureEnum {
    Update(Update),
    Screenshot(Screenshot),
}
//Using pattern matching
impl FeatureEnum {
    pub fn get_command(&self) -> String {
        match self {
            FeatureEnum::Update(f) => f.get_command(),
            FeatureEnum::Screenshot(f) => f.get_command(),
        }
    }

    pub async fn run(&self, connector: &mut Connector) {
        match self {
            FeatureEnum::Update(f) => f.run(connector).await,
            FeatureEnum::Screenshot(f) => f.run(connector).await,
        }
    }
}
pub fn find_feature_by_command(command: &str) -> Option<FeatureEnum> {
    for feature in FEATURES {
        if command == feature.get_command() {
            feature
        }
    }
    None
}
pub enum Result {
    SUCCESFUL,
    FAILED,
}
