use async_trait::async_trait;
use model::SweatHeroInterfaceIntegration;
use serde_json::json;
use workspaces::Contract;

pub const SWEAT_HERO: &str = "sweat_hero";

pub struct SweatHero<'a> {
    pub contract: &'a Contract,
}

#[async_trait]
impl SweatHeroInterfaceIntegration for SweatHero<'_> {
    async fn init(&self) -> anyhow::Result<()>
    where
        Self: Sized,
    {
        println!("▶️ Init contract");

        self.contract.call("init").max_gas().transact().await?.into_result()?;

        Ok(())
    }

    async fn initialize_with_name(&self, name: String) -> anyhow::Result<()>
    where
        Self: Sized,
    {
        println!("▶️ Init contract with name");

        self.contract
            .call("init_with_name")
            .args_json(json!({
                "name": name,
            }))
            .max_gas()
            .transact()
            .await?
            .into_result()?;

        Ok(())
    }

    async fn receive_name(&self) -> anyhow::Result<String> {
        println!("▶️ Init contract with name");

        let result = self
            .contract
            .call("receive_name")
            .max_gas()
            .transact()
            .await?
            .into_result()?;

        Ok(result.json()?)
    }

    async fn set_name(&mut self, name: String) -> anyhow::Result<()> {
        println!("▶️ Init contract with name");

        self.contract
            .call("set_name")
            .args_json(json!({
                "name": name,
            }))
            .max_gas()
            .transact()
            .await?
            .into_result()?;

        Ok(())
    }
}
