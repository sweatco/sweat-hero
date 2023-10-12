use async_trait::async_trait;
use model::SweatHeroInterfaceIntegration;
use near_sdk::AccountId;

use crate::sweat_hero_interface::{SweatHero, SWEAT_HERO};

pub type Context = integration_utils::context::Context<workspaces::network::Sandbox>;

#[async_trait]
pub trait IntegrationContracts {
    async fn account_with_name(&mut self, name: &str) -> anyhow::Result<AccountId>;
    async fn manager(&mut self) -> anyhow::Result<AccountId>;
    async fn alice(&mut self) -> anyhow::Result<AccountId>;
    fn sweat_hero(&self) -> SweatHero;
}

#[async_trait]
impl IntegrationContracts for Context {
    async fn account_with_name(&mut self, name: &str) -> anyhow::Result<AccountId> {
        let account = self.account(name).await?;
        Ok(AccountId::new_unchecked(account.id().to_string()))
    }

    async fn manager(&mut self) -> anyhow::Result<AccountId> {
        self.account_with_name("manager").await
    }

    async fn alice(&mut self) -> anyhow::Result<AccountId> {
        self.account_with_name("alice").await
    }

    fn sweat_hero(&self) -> SweatHero {
        SweatHero {
            contract: &self.contracts[SWEAT_HERO],
        }
    }
}

pub async fn prepare_contract() -> anyhow::Result<Context> {
    let mut context = Context::new(&[SWEAT_HERO]).await?;
    let manager = context.manager().await?;
    context.sweat_hero().new(manager).await?;
    Ok(context)
}
