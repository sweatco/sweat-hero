use integration_trait::make_integration_version;
use near_sdk::AccountId;

pub type TokenId = String;

#[make_integration_version]
pub trait SweatHeroInterface {
    fn new(owner_id: AccountId) -> Self;
}
