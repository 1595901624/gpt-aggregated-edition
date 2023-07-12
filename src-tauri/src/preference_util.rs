#![allow(dead_code)]
use std::path::PathBuf;

use log::warn;
use tauri::{
    api::path::{resolve_path, BaseDirectory},
    Env,
};

use crate::{
    constant,
    model::{
        extension_menu::{ExtensionMenu, ParentMenu},
        preference_model::{Preference, WindowMode},
    },
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
        current_page_url: Some("https://yiyan.baidu.com/".to_string()),
        exit_app: Some(false),
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

// /// 获取扩展的菜单信息
// pub fn get_extendsion_menu() {
//     if let Ok(current_exec) = std::env::current_exe() {
//         let file = current_exec.join("test.txt");
//         if !file.exists() {

//         }
//         match std::fs::write(file, "1111111111111111") {
//             Ok(_) => {
//                 dbg!("success");
//             },
//             Err(e) => {
//                 dbg!(e);
//             },
//         }
//     }
// }

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

pub fn set_app_preference(json: &str) -> bool {
    // let json = serde_json::to_string(p).unwrap();
    if let Ok(_) = std::fs::write(get_app_preference_path(), json) {
        return true;
    }
    return false;
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

/// 是否退出app
pub fn is_exit_app() -> bool {
    if let Ok(perference) = get_app_preference() {
        return perference.exit_app.unwrap_or_default();
    }
    return false;
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
        constant::PREFERENCE_EXIT_APP => {
            p.exit_app = Some(value.parse::<bool>().unwrap_or_default());
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
        warn!("读取配置文件失败");
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
            let ret = p
                .current_page_url
                .unwrap_or_else(|| String::from("https://yiyan.baidu.com/"));
            return ret;
        }
        constant::PREFERENCE_EXIT_APP => {
            let ret = p.exit_app.unwrap_or_default();
            return ret.to_string();
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

/// 读取自定义菜单目录(内置目录)
pub fn get_custom_menu_path() -> Option<PathBuf> {
    let context = tauri::generate_context!();
    if let Ok(res) = resolve_path(
        context.config(),
        context.package_info(),
        &Env::default(),
        "extensions/menu.json",
        Some(BaseDirectory::AppConfig),
    ) {
        return Some(res);
    }
    return None;
}

/// 读取自定义菜单(内置目录)
pub fn get_custom_menu_list() -> Option<Vec<ExtensionMenu>> {
    // if let Some(res) = app.path_resolver().resolve_resource("extensions/menu.json") {
    if let Some(res) = get_custom_menu_path() {
        // dbg!(res);
        if !res.exists() {
            // 不存在
            let _ = std::fs::create_dir_all((&res).parent().unwrap());
            let _ = std::fs::write(&res, "");
        }

        if let Ok(byte) = std::fs::read(res) {
            if let Ok(list) = serde_json::from_slice::<Vec<ExtensionMenu>>(&byte) {
                return Some(list);
            }
        }
    }
    return Some(vec![]);
}

/// 读取内置菜单
pub fn get_internal_menu_list() -> Option<Vec<ParentMenu>> {
    let content = include_str!("../resource/menu.json");
    if let Ok(list) = serde_json::from_str::<Vec<ParentMenu>>(content) {
        // info!("get_internal_menu_list {:?}", list);
        return Some(list);
    }
    return None;
}
