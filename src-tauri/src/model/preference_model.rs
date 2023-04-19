#![allow(dead_code)]
use serde::{Deserialize, Deserializer, Serialize};

/// 设置页模式
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Preference {
    // 窗口模式 0为窗口模式,1为任务栏模式,2侧边栏模式
    pub window_mode: WindowMode,

    // 启用内置脚本 默认为false
    #[serde(default)]
    #[serde(deserialize_with = "default_boolean")]
    pub enable_internal_script: Option<bool>,

    // 启用外置脚本 默认为false
    #[serde(default)]
    #[serde(deserialize_with = "default_boolean")]
    pub enable_extendsion_script: Option<bool>,

    // 当点击窗口外侧自动隐藏窗口，默认自动隐藏
    #[serde(default)]
    #[serde(deserialize_with = "default_boolean")]
    pub auto_hide_when_click_outside: Option<bool>,

    // 当前的访问页面的地址
    // #[serde(with = "false")]
    #[serde(default)]
    #[serde(deserialize_with = "default_page_url")]
    pub current_page_url: Option<String>,

    // 点击关闭退出应用
    #[serde(default)]
    #[serde(deserialize_with = "default_boolean")]
    pub exit_app: Option<bool>,
}

fn default_empty_string<'de, D>(d: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    Deserialize::deserialize(d).map(|x: Option<_>| x.unwrap_or(Some("".to_string())))
}

fn default_page_url<'de, D>(d: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    Deserialize::deserialize(d)
        .map(|x: Option<_>| x.unwrap_or(Some("https://yiyan.baidu.com/".to_string())))
}

fn default_boolean<'de, D>(d: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    Deserialize::deserialize(d).map(|x: Option<_>| x.unwrap_or(Some(false)))
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum WindowMode {
    Window,
    TaskBar,
    // 侧边栏
    SideBar,
    // QQ模式
    QQ
}

impl Default for WindowMode {
    fn default() -> Self {
        WindowMode::Window
    }
}
