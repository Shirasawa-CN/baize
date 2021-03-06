use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;

#[derive(Serialize, Deserialize)]
pub struct BaizeConfiguration {
    //代码行数
    pub number_of_codes: bool,
    //高亮设置
    pub highlight: bool,
    //启用/禁用插件系统
    pub status_plugins: bool,
    //是否插件调试模式（非用户使用，提供给插件开发者）
    pub debug_plugins: bool,
    //插件商店
    pub store_plugins: bool,
    //插件列表
    pub plugins: Vec<String>,
    //插件存放路径
    pub plugin_path: String,
}

pub async fn get_baize_configuration() -> Result<BaizeConfiguration> {
    let file = File::open("baize.json")?;
    let reader = BufReader::new(file);
    let config: BaizeConfiguration = serde_json::from_reader(reader)?;
    Ok(config)
}

#[cfg(test)]
mod test {

    #[test]
    fn test_get_baize_configuration() {
        use crate::api::configuration::get_baize_configuration;

        let _a = async move {
            let result = get_baize_configuration();
            assert!(result.await.is_ok());
        };
    }
}
