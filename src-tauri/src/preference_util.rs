use std::path::PathBuf;

use tauri::{
    api::path::{resolve_path, BaseDirectory},
    Env,
};

use crate::model::preference_model::{Preference, WindowMode};

/// 初始化默认配置
pub fn init_default_preference() {
    let path = get_app_preference_path();
    if path.exists() {
        return;
    }
    let parent = path.parent().unwrap();
    std::fs::create_dir_all(parent).unwrap();
    // std::fs::File::create(path).unwrap();
    let preference = Preference {
        window_mode: WindowMode::Window,
    };
    let json = serde_json::to_string(&preference).unwrap();
    let _ = std::fs::write(get_app_preference_path(), json);
}

/// 获取app的配置路径
pub fn get_app_preference_path() -> PathBuf {
    let context = tauri::generate_context!();
    return resolve_path(
        context.config(),
        context.package_info(),
        &Env::default(),
        "preference.json",
        Some(BaseDirectory::AppConfig),
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
    return perference.window_mode;
}

/// 设置模式
/// `mode` 1 是任务栏模式 其它是窗口模式
pub fn set_window_mode(mode: i32) -> bool {
    let perference = get_app_preference();
    if perference.is_err() {
        dbg!("读取配置文件失败");
        return false;
    }
    let mut p = perference.unwrap();
    if mode == 1 {
        p.window_mode = WindowMode::TaskBar;
    } else {
        p.window_mode = WindowMode::Window;
    }
    let json = serde_json::to_string(&p).unwrap();
    let _ = std::fs::write(get_app_preference_path(), json);
    return true;
}
