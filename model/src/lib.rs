pub mod contract_metadata;
pub mod token;
pub mod token_metadata;
pub mod token_view;

use integration_trait::make_integration_version;
use near_sdk::AccountId;

use crate::{contract_metadata::ContractMetadata, token_metadata::TokenMetadata, token_view::TokenView};

pub const SPEC: &str = "nft-1.0.0";
pub const NAME: &str = "Sweat Hero Legs";
pub const SYMBOL: &str = "LEGS";

pub type TokenId = String;

#[make_integration_version]
pub trait SweatHeroInterface {
    fn new(owner_id: AccountId) -> Self;
    fn nft_metadata(&self) -> ContractMetadata;
    fn nft_mint(&mut self, token_id: TokenId, metadata: TokenMetadata, receiver_id: AccountId);
    fn nft_token(&self, token_id: TokenId) -> Option<TokenView>;
    fn nft_tokens_for_owner(
        &self,
        account_id: AccountId,
        offset: Option<usize>,
        limit: Option<usize>,
    ) -> Vec<TokenView>;
}
