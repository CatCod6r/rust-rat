use update::Update;

use super::instance::Instance;

pub mod update;

#[derive(Debug)]
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
