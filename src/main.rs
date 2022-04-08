use anyhow::Result;
use baize::application::BaizeApplication;

#[tokio::main]
async fn main() -> Result<()> {
    let mut baize_application = BaizeApplication::new();
    baize_application.run().await?;
    Ok(())
}
