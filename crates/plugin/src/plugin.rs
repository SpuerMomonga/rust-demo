/// 插件管理 trait
pub trait PluginManager {
    /// 启动加载插件
    fn startup_load_plugins();

    /// 加载测试或本地插件
    fn load_plugin();

    /// 下载安装并启用插件
    fn download_plugin();

    /// 卸载插件
    fn uninstall_plugin();

    /// 启用插件
    fn enable_plugin();

    /// 禁用插件
    fn disable_plugin();

    /// 获取插件列表
    fn get_plugins();
}

pub struct PluginStore {}
