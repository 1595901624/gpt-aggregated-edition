use std::path::PathBuf;

use tauri::{
    api::{
        self,
        path::{resolve_path, BaseDirectory},
    },
    Env,
};

use crate::model::preference_model::{Preference, WindowMode};

/// 获取app的配置路径
pub fn get_app_preference_path() -> PathBuf {
    let context = tauri::generate_context!();
    return resolve_path(
        context.config(),
        context.package_info(),
        &Env::default(),
        "preference.json",
        Some(BaseDirectory::AppData),
    )
    .unwrap();
}

/// 从文件读取配置
pub fn get_app_preference() -> Result<Preference, String> {
    let path = get_app_preference_path();
    let file = std::fs::read(path);
    if file.is_err() {
        return Err("file is not exist!".to_string());
    }
    let file = file.unwrap();
    let result = serde_json::from_slice::<Preference>(&file);
    if result.is_err() {
        return Err("serialize error!".to_string());
    }
    return Ok(result.unwrap());
}

/// 获取窗口模式
pub fn get_window_mode() -> WindowMode {
    let perference = get_app_preference();
    if perference.is_err() {
        return WindowMode::Window;
    }
    let perference = perference.unwrap();
    return perference.window_mode.unwrap_or_else(|| WindowMode::Window);
}
