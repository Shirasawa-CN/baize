pub mod plug;
pub mod screen;
pub mod keyboard;
pub mod themes;

use crate::message::*;

use std::io::Read;
use std::fs::File;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BaizeConfiguration{
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
    //插件缓存路径
    pub plugin_cache_path: String,
}

pub fn read_baize_configuration() -> Result<Box<BaizeConfiguration>,&'static str>{
    let config_file = File::open("baize.json");
    match config_file{
        Ok(mut file) => {
            let mut config_string = String::new();
            file.read_to_string(&mut config_string);
            let config:BaizeConfiguration = serde_json::from_str(&config_string).unwrap();
            Ok(Box::from(config))
        },
        Err(_) => Err(ERROR_MSG_CAN_NOT_OPEN_FILE)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn testall() {
        use crate::api::read_baize_configuration;

        let result = read_baize_configuration();
        assert_eq!(result.is_ok(), true);
    }
}
