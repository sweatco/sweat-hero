#![cfg(test)]

use model::{token_metadata::TokenMetadata, SweatHeroInterfaceIntegration, NAME, SPEC, SYMBOL};

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

#[tokio::test]
async fn mint() -> anyhow::Result<()> {
    println!("👷🏽 Run mint test");

    let mut context = prepare_contract().await?;

    let alice = context.alice().await?;

    let token_id = "aaaa";

    context
        .sweat_hero()
        .nft_mint(
            token_id.to_string(),
            TokenMetadata::new("Cool legs", "Very very cool legs"),
            alice.clone(),
        )
        .await?;

    let token_view = context.sweat_hero().nft_token(token_id.to_string()).await?.unwrap();

    assert_eq!(token_view.token_id, token_id);
    assert_eq!(token_view.owner_id, alice);
    assert_eq!(token_view.metadata.title, "Cool legs");
    assert_eq!(token_view.metadata.description, "Very very cool legs");

    dbg!(&token_view);

    Ok(())
}
