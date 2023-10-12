use model::{SweatHeroInterfaceIntegration, NAME, SPEC, SYMBOL};

use crate::prepare::{prepare_contract, IntegrationContracts};

#[tokio::test]
async fn initialize() -> anyhow::Result<()> {
    println!("👷🏽 Run initialize test");

    let context = prepare_contract().await?;

    let metadata = context.sweat_hero().nft_metadata().await?;

    assert_eq!(metadata.spec, SPEC);
    assert_eq!(metadata.name, NAME);
    assert_eq!(metadata.symbol, SYMBOL);

    Ok(())
}
