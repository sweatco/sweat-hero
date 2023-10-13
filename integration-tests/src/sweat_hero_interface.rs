use std::mem::{size_of, MaybeUninit};

use async_trait::async_trait;
use model::{
    contract_metadata::ContractMetadata, legs::legs_metadata::LegsMetadata, token_metadata::TokenMetadata,
    token_view::TokenView, SweatHeroInterfaceIntegration, TokenId,
};
use near_sdk::{
    serde::{de::DeserializeOwned, Serialize},
    AccountId,
};
use serde_json::json;
use workspaces::Contract;

pub const SWEAT_HERO: &str = "sweat_hero";

pub struct SweatHero<'a> {
    pub contract: &'a Contract,
}

impl SweatHero<'_> {
    async fn call_contract<T: DeserializeOwned, P: Serialize>(&self, method: &str, args: P) -> anyhow::Result<T> {
        println!("▶️ {method}");

        let result = self
            .contract
            .call(method)
            .args_json(args)
            .max_gas()
            .transact()
            .await?
            .into_result()?;

        if size_of::<T>() == 0 {
            // For cases when return type is `()` and we don't need to parse result.
            // This call is safe for zero sized types.
            #[allow(clippy::uninit_assumed_init)]
            Ok(unsafe { MaybeUninit::uninit().assume_init() })
        } else {
            Ok(result.json()?)
        }
    }
}

#[async_trait]
impl SweatHeroInterfaceIntegration for SweatHero<'_> {
    async fn new(&self, owner_id: AccountId) -> anyhow::Result<()> {
        self.call_contract(
            "new",
            json!({
                "owner_id": owner_id,
            }),
        )
        .await
    }

    async fn nft_metadata(&self) -> anyhow::Result<ContractMetadata> {
        self.call_contract("nft_metadata", ()).await
    }

    async fn nft_mint(
        &mut self,
        token_id: TokenId,
        metadata: TokenMetadata,
        legs_metadata: LegsMetadata,
        receiver_id: AccountId,
    ) -> anyhow::Result<()> {
        self.call_contract(
            "nft_mint",
            json!({
                 "token_id": token_id,
                "metadata": metadata,
                "legs_metadata": legs_metadata,
                "receiver_id": receiver_id,
            }),
        )
        .await
    }

    async fn nft_token(&self, token_id: TokenId) -> anyhow::Result<Option<TokenView>> {
        self.call_contract(
            "nft_token",
            json!({
                "token_id": token_id,
            }),
        )
        .await
    }

    async fn nft_tokens_for_owner(
        &self,
        account_id: AccountId,
        from_index: Option<usize>,
        limit: Option<usize>,
    ) -> anyhow::Result<Vec<TokenView>> {
        self.call_contract(
            "nft_tokens_for_owner",
            json!({
                "account_id": account_id,
                "from_index": from_index,
                "limit": limit,
            }),
        )
        .await
    }

    async fn legs_metadata(&self, token_id: TokenId) -> anyhow::Result<Option<LegsMetadata>> {
        self.call_contract(
            "legs_metadata",
            json!({
                "token_id": token_id,
            }),
        )
        .await
    }
}
