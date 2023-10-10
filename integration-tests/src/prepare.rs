use integration_utils::context::Context;
use model::SweatHeroInterfaceIntegration;

use crate::sweat_hero_interface::{SweatHero, SWEAT_HERO};

pub trait IntegrationContracts {
    fn sweat_hero(&self) -> SweatHero;
}

impl IntegrationContracts for Context {
    fn sweat_hero(&self) -> SweatHero {
        SweatHero {
            contract: &self.contracts[SWEAT_HERO],
        }
    }
}

pub async fn prepare_contract() -> anyhow::Result<Context> {
    let context = Context::new(&[SWEAT_HERO]).await?;
    context.sweat_hero().init().await?;
    Ok(context)
}
