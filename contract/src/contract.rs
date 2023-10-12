use model::{contract_metadata::ContractMetadata, TokenId};
use near_sdk::{
    borsh,
    borsh::{BorshDeserialize, BorshSerialize},
    collections::LazyOption,
    near_bindgen,
    store::{LookupMap, UnorderedMap, UnorderedSet},
    AccountId, BorshStorageKey, PanicOnDefault,
};

use crate::{token::Token, token_metadata::TokenMetadata};

#[derive(BorshStorageKey, BorshSerialize)]
pub(crate) enum StorageKey {
    TokensPerOwner,
    TokensById,
    TokenMetadataById,
    Metadata,
}

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
