use syn_ack::*;

#[tokio::test]
async fn test_init_application() -> anyhow::Result<()> {
    test_utils::env().setup();
    start().await;
    Ok(())
}
