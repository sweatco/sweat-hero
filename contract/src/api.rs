use model::{
    contract_metadata::ContractMetadata, token::Token, token_metadata::TokenMetadata, token_view::TokenView,
    SweatHeroInterface, TokenId, NAME, SPEC, SYMBOL,
};
use near_sdk::{collections::LazyOption, env, near_bindgen, store::LookupMap, AccountId};

use crate::{
    contract::StorageKey,
    event::{emit, Event, MintData},
    internal::refund_deposit,
    Contract, ContractExt,
};

#[near_bindgen]
impl SweatHeroInterface for Contract {
    #[init]
    fn new(owner_id: AccountId) -> Self {
        Self {
            tokens_per_owner: LookupMap::new(StorageKey::TokensPerOwner),
            tokens_by_id: LookupMap::new(StorageKey::TokensById),
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

    #[payable]
    fn nft_mint(&mut self, token_id: TokenId, metadata: TokenMetadata, receiver_id: AccountId) {
        //measure the initial storage being used on the contract
        let initial_storage_usage = env::storage_usage();

        let token = Token {
            owner_id: receiver_id.clone(),
            metadata,
        };

        //insert the token ID and token struct and make sure that the token doesn't exist
        assert!(
            self.tokens_by_id.insert(token_id.clone(), token).is_none(),
            "Token already exists"
        );

        //call the internal method for adding the token to the owner
        self.internal_add_token_to_owner(&receiver_id, &token_id);

        emit(Event::NFTMint(MintData { receiver_id, token_id }));

        //calculate the required storage which was the used - initial
        let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;

        //refund any excess storage if the user attached too much. Panic if they didn't attach enough to cover the required.
        refund_deposit(required_storage_in_bytes);
    }

    fn nft_token(&self, token_id: TokenId) -> Option<TokenView> {
        self.tokens_by_id
            .get(&token_id)
            .map(|token| TokenView::from_token(token, token_id))
    }

    fn nft_tokens_for_owner(
        &self,
        account_id: AccountId,
        from_index: Option<usize>,
        limit: Option<usize>,
    ) -> Vec<TokenView> {
        let Some(tokens) = self.tokens_per_owner.get(&account_id) else {
            return vec![];
        };

        tokens
            .iter()
            .skip(from_index.unwrap_or(0))
            .take(limit.unwrap_or(50))
            .map(|token_id| self.nft_token(token_id.clone()).unwrap())
            .collect()
    }
}
