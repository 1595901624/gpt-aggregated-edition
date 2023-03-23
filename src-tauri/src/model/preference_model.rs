use serde::{Deserialize, Serialize};

/// 设置页模式
#[derive(Serialize, Deserialize, Debug)]
pub struct Preference {
    // 窗口模式 0为窗口模式，1为任务栏模式
    pub window_mode: WindowMode,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum WindowMode {
    Window,
    TaskBar,
}
