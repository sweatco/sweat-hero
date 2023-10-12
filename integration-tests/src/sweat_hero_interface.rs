use async_trait::async_trait;
use model::{
    contract_metadata::ContractMetadata, token_metadata::TokenMetadata, token_view::TokenView,
    SweatHeroInterfaceIntegration, TokenId,
};
use near_sdk::AccountId;
use serde_json::json;
use workspaces::Contract;

pub const SWEAT_HERO: &str = "sweat_hero";

pub struct SweatHero<'a> {
    pub contract: &'a Contract,
}

#[async_trait]
impl SweatHeroInterfaceIntegration for SweatHero<'_> {
    async fn new(&self, owner_id: AccountId) -> anyhow::Result<()> {
        println!("▶️ Init contract");

        self.contract
            .call("new")
            .args_json(json!({
                "owner_id": owner_id,
            }))
            .max_gas()
            .transact()
            .await?
            .into_result()?;

        Ok(())
    }

    async fn nft_metadata(&self) -> anyhow::Result<ContractMetadata> {
        println!("▶️ nft_metadata");

        let result = self
            .contract
            .call("nft_metadata")
            .max_gas()
            .transact()
            .await?
            .into_result()?;

        Ok(result.json()?)
    }

    async fn nft_mint(
        &mut self,
        token_id: TokenId,
        metadata: TokenMetadata,
        receiver_id: AccountId,
    ) -> anyhow::Result<()> {
        println!("▶️ nft_mint");

        self.contract
            .call("nft_mint")
            .args_json(json!({
                "token_id": token_id,
                "metadata": metadata,
                "receiver_id": receiver_id,
            }))
            .max_gas()
            .transact()
            .await?
            .into_result()?;

        Ok(())
    }

    async fn nft_token(&self, token_id: TokenId) -> anyhow::Result<Option<TokenView>> {
        println!("▶️ nft_token");

        let result = self
            .contract
            .call("nft_token")
            .args_json(json!({
                "token_id": token_id,
            }))
            .max_gas()
            .transact()
            .await?
            .into_result()?;

        Ok(result.json()?)
    }

    async fn nft_tokens_for_owner(
        &self,
        account_id: AccountId,
        from_index: Option<usize>,
        limit: Option<usize>,
    ) -> anyhow::Result<Vec<TokenView>> {
        println!("▶️ nft_tokens_for_owner");

        let result = self
            .contract
            .call("nft_tokens_for_owner")
            .args_json(json!({
                "account_id": account_id,
                "from_index": from_index,
                "limit": limit,
            }))
            .max_gas()
            .transact()
            .await?
            .into_result()?;

        Ok(result.json()?)
    }
}
