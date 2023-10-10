use integration_trait::make_integration_version;

#[make_integration_version]
pub trait SweatHeroInterface {
    fn init() -> Self
    where
        Self: Sized;

    fn initialize_with_name(name: String) -> Self
    where
        Self: Sized;

    fn receive_name(&self) -> String;
    fn set_name(&mut self, name: String);
}
