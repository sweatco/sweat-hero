#[tokio::test]
async fn happy_flow() -> anyhow::Result<()> {
    use model::SweatHeroInterfaceIntegration;

    use crate::prepare::{prepare_contract, IntegrationContracts};

    println!("👷🏽 Run happy flow test");

    let context = prepare_contract().await?;

    assert_eq!(context.sweat_hero().receive_name().await?, "Default name");

    context.sweat_hero().set_name("New name".to_string()).await?;

    assert_eq!(context.sweat_hero().receive_name().await?, "New name");

    Ok(())
}
