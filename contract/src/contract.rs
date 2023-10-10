use near_sdk::{
    borsh,
    borsh::{BorshDeserialize, BorshSerialize},
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
