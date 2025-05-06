use super::instance::Instance;
use once_cell::sync::Lazy;
use screenshot::Screenshot;
use update::Update;

pub mod screenshot;
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
    pub fn get_name(&self) -> String {
        match self {
            FeatureEnum::Update(f) => f.get_name(),
            FeatureEnum::Screenshot(f) => f.get_name(),
        }
    }

    pub async fn run(&self, instance: &mut Instance) {
        match self {
            FeatureEnum::Update(f) => f.run(instance).await,
            FeatureEnum::Screenshot(f) => f.run(instance).await,
        }
    }
}
