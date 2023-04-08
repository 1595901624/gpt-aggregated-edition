use std::path::PathBuf;

use tauri::{
    api::path::{resolve_path, BaseDirectory},
    Env,
};

use crate::{
    constant,
    model::preference_model::{Preference, WindowMode},
};

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
        enable_internal_script: Some(true),
        enable_extendsion_script: Some(false),
        auto_hide_when_click_outside: Some(true),
        ..Default::default()
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

/// app config 根路径
pub fn get_app_config_root_path() -> PathBuf {
    let context = tauri::generate_context!();
    return resolve_path(
        context.config(),
        context.package_info(),
        &Env::default(),
        "",
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
    } else if mode == 2 {
        p.window_mode = WindowMode::SideBar;
    } else {
        p.window_mode = WindowMode::Window;
    }
    let json = serde_json::to_string(&p).unwrap();
    let _ = std::fs::write(get_app_preference_path(), json);
    return true;
}

/// 是否启用内置脚本
pub fn is_enable_internal_script() -> bool {
    let perference = get_app_preference();
    if perference.is_err() {
        return false;
    }
    let perference = perference.unwrap();
    return perference.enable_internal_script.unwrap_or_else(|| false);
}

/// 启用或者关闭内置脚本
pub fn enable_internal_script(enable: bool) -> bool {
    let perference = get_app_preference();
    if perference.is_err() {
        dbg!("读取配置文件失败");
        return false;
    }
    let mut p = perference.unwrap();
    p.enable_internal_script = Some(enable);
    let json = serde_json::to_string(&p).unwrap();
    let _ = std::fs::write(get_app_preference_path(), json);
    return true;
}

/// 设置设置项
pub fn set_preference(key: i32, value: &str) -> bool {
    let perference = get_app_preference();
    if perference.is_err() {
        dbg!("读取配置文件失败");
        return false;
    }
    let mut p = perference.unwrap();
    match key {
        constant::PREFERENCE_AUTO_HIDE_WHEN_CLICK_OUTSIDE => {
            // 设置自动隐藏
            p.auto_hide_when_click_outside = Some(value.parse::<bool>().unwrap_or_else(|_| false));
        }
        constant::PREFERENCE_CURRENT_PAGE_URL => {
            p.current_page_url = Some(value.to_string());
        }
        _ => {}
    }
    // 写入文件
    let json = serde_json::to_string(&p).unwrap();
    let _ = std::fs::write(get_app_preference_path(), json);
    return true;
}

/// 获取设置项
pub fn get_preference(key: i32, value: &str) -> String {
    let perference = get_app_preference();
    if perference.is_err() {
        dbg!("读取配置文件失败");
        return value.into();
    }
    let p = perference.unwrap();
    match key {
        constant::PREFERENCE_AUTO_HIDE_WHEN_CLICK_OUTSIDE => {
            // 设置自动隐藏
            // p.auto_hide_when_click_outside = Some(value.parse::<bool>().unwrap_or_else(|_|false));
            let ret = p
                .auto_hide_when_click_outside
                .unwrap_or_else(|| value.parse::<bool>().unwrap_or_else(|_| true));
            return ret.to_string();
        }
        constant::PREFERENCE_CURRENT_PAGE_URL => {
            let ret = p.current_page_url.unwrap_or_else(|| String::from("https://yiyan.baidu.com/"));
            return ret;
        }
        _ => {}
    }
    return value.into();
}

/// 任务栏模式下点击外侧自动隐藏
pub fn auto_hide_when_click_outside() -> bool {
    return get_preference(constant::PREFERENCE_AUTO_HIDE_WHEN_CLICK_OUTSIDE, "true")
        .parse::<bool>()
        .unwrap_or_else(|_| true);
}

// 是否启用扩展脚本
// pub fn is_enable_extendsion_script() {}
