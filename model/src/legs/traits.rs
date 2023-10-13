use near_sdk::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Traits {
    pub flare: String,
    pub skin: String,
    pub pants: String,
    pub shoes: String,
}
