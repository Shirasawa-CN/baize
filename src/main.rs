pub mod api;
pub mod keyboard;
pub mod server;
use crate::api::read_baize_configuration;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let _baize_config = read_baize_configuration();

    Ok(())
}
