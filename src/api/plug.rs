use crate::api::read_baize_configuration;

use anyhow::Result;
use std::process::Command;

pub fn download_plug() -> Result<()> {
    let config = read_baize_configuration()?;

    let url = format!("https://github.com/");
    let path = format!("{}", config.plugin_path);

    for plugin in config.plugins {
        let plugin_name = plugin.split("/").last().unwrap();
        let plugin_path = format!("{}/{}", path, plugin_name);
        let plugin_url = format!("{}/{}", url, plugin);

        if std::path::Path::new(&plugin_path).exists() {
            Command::new("git")
                .args(&["clone", &plugin_url, &plugin_path])
                .output()
                .expect("clone失败，请检查网络环境");
        }
    }
    Ok(())
}

pub fn upgrade_plug() -> Result<()> {
    let config = read_baize_configuration()?;

    let url = format!("https://github.com/");
    let path = format!("{}", config.plugin_path);

    for plugin in config.plugins {
        let plugin_name = plugin.split("/").last().unwrap();
        let plugin_path = format!("{}/{}", path, plugin_name);
        let plugin_url = format!("{}/{}", url, plugin);

        if std::path::Path::new(&plugin_path).exists() {
            Command::new("git")
                .args(&["pull", &plugin_url, &plugin_path])
                .output()
                .expect("pull失败，请先检查网络环境或是否安装了所有插件");
        }
    }
    Ok(())
}