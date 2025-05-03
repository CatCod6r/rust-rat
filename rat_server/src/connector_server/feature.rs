use super::instance::Instance;

pub mod update;

pub trait Feature: std::fmt::Debug + Send + Sync {
    fn get_name(&self) -> String;
}
