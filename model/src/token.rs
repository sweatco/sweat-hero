use near_sdk::{
    borsh,
    borsh::{BorshDeserialize, BorshSerialize},
    AccountId,
};

use crate::token_metadata::TokenMetadata;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Token {
    pub owner_id: AccountId,
    pub metadata: TokenMetadata,
}
