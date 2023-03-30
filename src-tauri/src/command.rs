use crate::{preference_util};

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
    return preference_util::set_preference(key, value);
}

/// 获取设置项
#[tauri::command]
pub fn get_preference_handler(key: i32, value: &str) -> String {
    return preference_util::get_preference(key, value);
}
