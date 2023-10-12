pub mod contract_metadata;

use integration_trait::make_integration_version;
use near_sdk::AccountId;

use crate::contract_metadata::ContractMetadata;

pub const SPEC: &str = "nft-1.0.0";
pub const NAME: &str = "Sweat Hero Legs";
pub const SYMBOL: &str = "LEGS";

pub type TokenId = String;

#[make_integration_version]
pub trait SweatHeroInterface {
    fn new(owner_id: AccountId) -> Self;
    fn nft_metadata(&self) -> ContractMetadata;
}
