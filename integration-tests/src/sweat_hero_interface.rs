use async_trait::async_trait;
use model::SweatHeroInterfaceIntegration;
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
}
