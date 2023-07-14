// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#[macro_use]
extern crate lazy_static;
mod command;
mod model;
mod plugin;
mod preference_util;
mod util;
mod window;

use model::constant::{self};
use tauri::{generate_handler, SystemTray};
use tauri_plugin_log::LogTarget;

fn main() {
    // 初始化设置项
    preference_util::init_default_preference();

    // 初始化右下角菜单
    let tray = SystemTray::new().with_menu(window::menu::create_tary_menu());

    let context = tauri::generate_context!();

    // test2();

    // 初始化窗口
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([LogTarget::LogDir, LogTarget::Stdout, LogTarget::Webview])
                .build(),
        )
        .invoke_handler(generate_handler![
            command::greet,
            command::get_window_mode_handler,
            command::set_window_mode_handler,
            command::is_enable_internal_script_handler,
            command::enable_internal_script_handler,
            command::create_markdown_handler,
            command::set_preference_handler,
            command::get_preference_handler,
            command::get_app_preference_handler,
            command::set_app_preference_handler,
            // command::create_docx_handler,
            command::create_markdown_handler,
            command::query_extension_menus_handler,
            command::add_extension_menu_item_handler,
            command::add_extension_menu_list_handler,
            command::edit_extension_menu_item_handler,
            command::delete_extension_menu_item_handler,
            command::delete_extension_menu_all_handler,
        ])
        .plugin(tauri_plugin_positioner::init())
        .setup(window::setup::setup)
        .system_tray(tray)
        // 窗口监听
        .on_window_event(window::menu::on_window_event)
        .on_menu_event(window::menu::on_window_event_handler)
        // 任务栏菜单监听
        .on_system_tray_event(window::menu::on_tray_event)
        .run(context)
        .expect("error while running tauri application");
}