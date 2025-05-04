use super::instance::Instance;
use once_cell::sync::Lazy;
use update::Update;

pub mod update;

pub static FEATURES: Lazy<Vec<FeatureEnum>> =
    Lazy::new(|| vec![FeatureEnum::Update(Update::new())]);
#[derive(Debug, Clone)]

pub enum FeatureEnum {
    Update(Update),
}
//Using pattern matching
impl FeatureEnum {
    pub fn get_name(&self) -> String {
        match self {
            FeatureEnum::Update(f) => f.get_name(),
        }
    }

    pub async fn run(&self, instance: &mut Instance) {
        match self {
            FeatureEnum::Update(f) => f.run(instance).await,
        }
    }
}
