use syn_ack::*;

#[tokio::test]
async fn test_init_application() -> anyhow::Result<()> {
    test_utils::setup();
    start().await;
    Ok(())
}
