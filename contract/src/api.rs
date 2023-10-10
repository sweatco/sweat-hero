use model::SweatHeroInterface;
use near_sdk::near_bindgen;

use crate::{Contract, ContractExt};

#[near_bindgen]
impl SweatHeroInterface for Contract {
    #[init]
    fn init() -> Self {
        Self {
            name: "Default name".to_string(),
        }
    }

    #[init]
    fn initialize_with_name(name: String) -> Self {
        Self { name }
    }

    fn receive_name(&self) -> String {
        self.name.clone()
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }
}
