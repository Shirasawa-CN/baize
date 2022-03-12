pub mod plug;
pub mod screen;
pub mod keyboard;
pub mod themes;

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

#[cfg(test)]
mod tests {
    #[test]
    fn configuration_test() {
        use crate::api::BaizeConfiguration;
        let json = r#"
        {
            "number_of_codes": true,
            "highlight": true,
            "status_plugins": true,
            "debug_plugins": true,
            "store_plugins": true,
            "plugins": [
                "plugin1",
                "plugin2"
            ],
            "plugin_path": "plugin_path",
            "plugin_cache_path": "plugin_cache_path"
        }"#;

        let test:BaizeConfiguration = serde_json::from_str(json).unwrap();
        assert_eq!(test.number_of_codes, true);
        assert_eq!(test.highlight, true);
        assert_eq!(test.status_plugins, true);
        assert_eq!(test.store_plugins, true);
        assert_eq!(test.debug_plugins, true);
        assert_eq!(test.plugins.len(), 2);
        assert_eq!(test.plugin_path, "plugin_path");
        assert_eq!(test.plugin_cache_path, "plugin_cache_path");
    }
}
