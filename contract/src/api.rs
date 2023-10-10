use model::SweatHeroInterface;
use near_sdk::{near_bindgen, AccountId};

use crate::{Contract, ContractExt};

#[near_bindgen]
impl SweatHeroInterface for Contract {
    #[init]
    fn new(owner_id: AccountId) -> Self {
        todo!()
    }
}
