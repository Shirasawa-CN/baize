pub mod plug;
pub mod screen;
pub mod keyboard;
pub mod themes;

struct Configuration{
    //代码行数
    pub number_of_codes: u32,
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