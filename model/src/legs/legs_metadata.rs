use near_sdk::serde::{Deserialize, Serialize};

use crate::legs::{rarity::Rarity, traits::Traits};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct LegsMetadata {
    pub rarity: Rarity,
    pub power: u32,
    pub energy: u32,
    pub energy_max: u32,
    pub level: u32,
    pub battles: u32,
    pub traits: Traits,
}

impl LegsMetadata {
    pub fn new_test() -> Self {
        LegsMetadata {
            rarity: Rarity::Legendary,
            power: 10,
            energy: 2,
            energy_max: 3,
            level: 5,
            battles: 10,
            traits: Traits {
                flare: "cool flare".to_string(),
                skin: "cool skin".to_string(),
                pants: "cool pants".to_string(),
                shoes: "cool shoes".to_string(),
            },
        }
    }
}
