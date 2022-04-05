use anyhow::{ Result};
use baize::application::BaizeApplication;

#[tokio::main]
async fn main() -> Result<()> {
    BaizeApplication::run().await?;
    Ok(())
}
