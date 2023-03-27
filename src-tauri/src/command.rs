use crate::preference_util;

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