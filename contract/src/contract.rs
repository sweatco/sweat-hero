use model::{contract_metadata::ContractMetadata, token::Token, TokenId};
use near_sdk::{
    borsh,
    borsh::{BorshDeserialize, BorshSerialize},
    collections::LazyOption,
    near_bindgen,
    store::{LookupMap, UnorderedSet},
    AccountId, BorshStorageKey, CryptoHash, PanicOnDefault,
};

#[derive(BorshStorageKey, BorshSerialize)]
pub(crate) enum StorageKey {
    TokensPerOwner,
    TokenPerOwnerInner { account_id_hash: CryptoHash },
    TokensById,
    Metadata,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub owner_id: AccountId,

    pub tokens_per_owner: LookupMap<AccountId, UnorderedSet<TokenId>>,

    pub tokens_by_id: LookupMap<TokenId, Token>,

    pub metadata: LazyOption<ContractMetadata>,
}
