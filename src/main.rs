use baize::application::BaizeApplication;
use baize_core::*;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let mut baize_application = BaizeApplication::new();
    baize_application.run().await?;
    Ok(())
}
