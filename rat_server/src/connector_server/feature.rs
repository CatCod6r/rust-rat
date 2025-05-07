use super::instance::Instance;
use cmd::Cmd;
use once_cell::sync::Lazy;
use screenshot::Screenshot;
use update::Update;

pub mod cmd;
pub mod screenshot;
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
    pub fn get_name(&self) -> String {
        match self {
            FeatureEnum::Update(f) => f.get_name(),
            FeatureEnum::Screenshot(f) => f.get_name(),
            FeatureEnum::Cmd(f) => f.get_name(),
        }
    }

    pub async fn run(&self, instance: &mut Instance) {
        match self {
            FeatureEnum::Update(f) => f.run(instance).await,
            FeatureEnum::Screenshot(f) => f.run(instance).await,
            FeatureEnum::Cmd(f) => f.run(instance).await,
        }
    }
}
