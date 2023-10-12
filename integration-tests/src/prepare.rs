use async_trait::async_trait;
use integration_utils::context::Context;
use model::SweatHeroInterfaceIntegration;
use near_sdk::AccountId;

use crate::sweat_hero_interface::{SweatHero, SWEAT_HERO};

#[async_trait]
pub trait IntegrationContracts {
    async fn manager(&mut self) -> anyhow::Result<AccountId>;
    fn sweat_hero(&self) -> SweatHero;
}

#[async_trait]
impl IntegrationContracts for Context {
    async fn manager(&mut self) -> anyhow::Result<AccountId> {
        let account = self.account("manager").await?;
        Ok(AccountId::new_unchecked(account.id().to_string()))
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
