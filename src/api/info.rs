use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;

#[derive(Serialize, Deserialize)]
pub struct BaizeConfiguration {
    // baize version
    pub version: String,
    // 动态库版本
    pub lib_version: String,
    // 已安装插件列表
    pub plugins: Vec<String>,
    // 启用/禁用插件系统
    pub status_plugins: bool,
}

pub async fn get_baize_info() -> Result<Box<BaizeInfo>> {
    let file = File::open("info.json")?;
    let reader = BufReader::new(file);
    let info: BaizeConfiguration = serde_json::from_reader(reader)?;
    Ok(Box::new(info))
}

#[cfg(test)]
mod test {

    #[test]
    fn test_get_baize_info() {
        use crate::api::get_baize_info;

        let _a = async move {
            let result = get_baize_info();
            assert!(result.await.is_ok());
        };
    }
}