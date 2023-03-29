use serde::{Deserialize, Serialize};

/// 设置页模式
#[derive(Serialize, Deserialize, Debug)]
pub struct Preference {
    // 窗口模式 0为窗口模式,1为任务栏模式,2侧边栏模式
    pub window_mode: WindowMode,

    // 启用内置脚本
    pub enable_internal_script: Option<bool>,

    // 启用外置脚本
    pub enable_extendsion_script: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum WindowMode {
    Window,
    TaskBar,
    // 侧边栏
    SideBar,
}
