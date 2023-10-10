use model::ContractNameInterface;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    near_bindgen,
    serde::Serialize,
    PanicOnDefault,
};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize, PanicOnDefault)]
#[serde(crate = "near_sdk::serde")]
pub struct Contract {
    pub name: String,
}

#[near_bindgen]
impl ContractNameInterface for Contract {
    #[init]
    #[must_use]
    fn init() -> Self {
        Self {
            name: "Default name".to_string(),
        }
    }

    #[init]
    #[must_use]
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
