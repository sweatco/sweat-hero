use integration_utils::context::Context;
use model::SweatHeroInterfaceIntegration;
use near_sdk::AccountId;
use workspaces::Account;

use crate::sweat_hero_interface::{SweatHero, SWEAT_HERO};

pub trait IntegrationContracts {
    fn manager(&self) -> AccountId;
    fn sweat_hero(&self) -> SweatHero;
}

impl IntegrationContracts for Context {
    fn manager(&self) -> AccountId {
        self.account("manager")
    }

    fn sweat_hero(&self) -> SweatHero {
        SweatHero {
            contract: &self.contracts[SWEAT_HERO],
        }
    }
}

pub async fn prepare_contract() -> anyhow::Result<Context> {
    let context = Context::new(&[SWEAT_HERO]).await?;
    context.sweat_hero().new(context.manager().id().clone()).await?;
    Ok(context)
}
