use crate::{constant, preference_util};

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

/// 获取
#[tauri::command]
pub fn get_window_mode_handler() -> i32 {
    preference_util::get_window_mode() as i32
}

/// 设置模式
#[tauri::command]
pub fn set_window_mode_handler(mode: i32) {
    preference_util::set_window_mode(mode);
}

#[tauri::command]
pub fn is_enable_internal_script_handler() -> bool {
    preference_util::is_enable_internal_script()
}

#[tauri::command]
pub fn enable_internal_script_handler(enable: bool) {
    preference_util::enable_internal_script(enable);
}

/// 设置设置项
#[tauri::command]
pub fn set_preference_handler(key: i32, value: &str) -> bool {
    let perference = preference_util::get_app_preference();
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
        _ => {}
    }
    // 写入文件
    let json = serde_json::to_string(&p).unwrap();
    let _ = std::fs::write(preference_util::get_app_preference_path(), json);
    return true;
}

/// 获取设置项
#[tauri::command]
pub fn get_preference_handler(key: i32, value: &str) -> String {
    let perference = preference_util::get_app_preference();
    if perference.is_err() {
        dbg!("读取配置文件失败");
        return value.into();
    }
    let p = perference.unwrap();
    match key {
        constant::PREFERENCE_AUTO_HIDE_WHEN_CLICK_OUTSIDE => {
            // 设置自动隐藏
            // p.auto_hide_when_click_outside = Some(value.parse::<bool>().unwrap_or_else(|_|false));
            let ret = p.auto_hide_when_click_outside.unwrap_or_else(|| value.parse::<bool>().unwrap_or_else(|_| true));
            return ret.to_string();
        }
        _ => {}
    }
    return value.into();
}
