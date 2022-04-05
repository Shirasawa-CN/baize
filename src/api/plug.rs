use crate::api::configuration::get_baize_configuration;

use anyhow::Result;
use std::process::Command;

pub async fn download_plug() -> Result<()> {
    let config = get_baize_configuration().await?;

    let url = format!("https://github.com/");
    let path = format!("{}", config.plugin_path);

    for plugin in config.plugins {
        let plugin_name = plugin.split('/').last().unwrap();
        let plugin_path = format!("{}/{}", path, plugin_name);
        let plugin_url = format!("{}/{}", url, plugin);

        if std::path::Path::new(&plugin_path).exists() {
            Command::new("git")
                .args(&["clone", &plugin_url, &plugin_path])
                .output()?;
        }
    }
    Ok(())
}

pub async fn upgrade_plug() -> Result<()> {
    let config = get_baize_configuration().await?;

    let url = format!("https://github.com/");
    let path = format!("{}", config.plugin_path);

    for plugin in config.plugins {
        let plugin_name = plugin.split('/').last().unwrap();
        let plugin_path = format!("{}/{}", path, plugin_name);
        let plugin_url = format!("{}/{}", url, plugin);

        if std::path::Path::new(&plugin_path).exists() {
            Command::new("git")
                .args(&["pull", &plugin_url, &plugin_path])
                .output()?;
        }
    }
    Ok(())
}

pub async fn run_plug() -> Result<()> {
    let config = get_baize_configuration().await?;

    let path = format!("{}", config.plugin_path);
    for plugin in config.plugins {
        let plugin_name = plugin.split('/').last().unwrap();
        let plugin_path = format!("{}/{}", path, plugin_name);

        if std::path::Path::new(&plugin_path).exists() {
            Command::new("./").args(&[&plugin_name, "main"]).output()?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::api::plug::{download_plug, upgrade_plug};

    #[test]
    fn test_download_plug() {
        let _a = async move {
            let result = download_plug();
            assert!(result.await.is_ok());
        };
    }

    #[test]
    fn test_upgrade_plug() {
        let _a = async move {
            let result = upgrade_plug();
            assert!(result.await.is_ok());
        };
    }
}
