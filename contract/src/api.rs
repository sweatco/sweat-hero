use model::{contract_metadata::ContractMetadata, SweatHeroInterface, NAME, SPEC, SYMBOL};
use near_sdk::{
    collections::LazyOption,
    near_bindgen,
    store::{LookupMap, UnorderedMap},
    AccountId,
};

use crate::{contract::StorageKey, Contract, ContractExt};

#[near_bindgen]
impl SweatHeroInterface for Contract {
    #[init]
    fn new(owner_id: AccountId) -> Self {
        Self {
            tokens_per_owner: LookupMap::new(StorageKey::TokensPerOwner),
            tokens_by_id: LookupMap::new(StorageKey::TokensById),
            token_metadata_by_id: UnorderedMap::new(StorageKey::TokenMetadataById),
            owner_id,
            metadata: LazyOption::new(
                StorageKey::Metadata,
                Some(&ContractMetadata {
                    spec: SPEC.to_string(),
                    name: NAME.to_string(),
                    symbol: SYMBOL.to_string(),
                    icon: None,
                    base_uri: None,
                    reference: None,
                    reference_hash: None,
                }),
            ),
        }
    }

    fn nft_metadata(&self) -> ContractMetadata {
        self.metadata.get().unwrap()
    }
}
