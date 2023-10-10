use model::TokenId;
use near_sdk::{
    borsh,
    borsh::{BorshDeserialize, BorshSerialize},
    collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet},
    near_bindgen, AccountId, PanicOnDefault,
};

use crate::{contract_metadata::ContractMetadata, token::Token, token_metadata::TokenMetadata};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    //contract owner
    pub owner_id: AccountId,

    pub tokens_per_owner: LookupMap<AccountId, UnorderedSet<TokenId>>,

    pub tokens_by_id: LookupMap<TokenId, Token>,

    pub token_metadata_by_id: UnorderedMap<TokenId, TokenMetadata>,

    pub metadata: LazyOption<ContractMetadata>,
}
