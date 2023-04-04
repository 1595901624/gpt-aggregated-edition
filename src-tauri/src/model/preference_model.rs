use serde::{Deserialize, Serialize};

/// 设置页模式
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Preference {
    // 窗口模式 0为窗口模式,1为任务栏模式,2侧边栏模式
    pub window_mode: WindowMode,

    // 启用内置脚本 默认为false
    pub enable_internal_script: Option<bool>,

    // 启用外置脚本 默认为false
    pub enable_extendsion_script: Option<bool>,

    // 当点击窗口外侧自动隐藏窗口，默认自动隐藏
    pub auto_hide_when_click_outside: Option<bool>,

    // 当前的访问页面的地址
    pub current_page_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum WindowMode {
    Window,
    TaskBar,
    // 侧边栏
    SideBar,
}

impl Default for WindowMode {
    fn default() -> Self {
        WindowMode::Window
    }
}
