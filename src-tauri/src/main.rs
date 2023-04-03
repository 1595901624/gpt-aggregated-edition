// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod command;
mod menu;
mod model;
mod plugin;
mod preference_util;

use model::{constant, preference_model::WindowMode};
use tauri::{generate_handler, GlobalShortcutManager, Manager, PhysicalSize, SystemTray};
use tauri_plugin_positioner::{Position, WindowExt};

fn main() {
    // 初始化设置项
    preference_util::init_default_preference();

    // 初始化右下角菜单
    let tray = SystemTray::new().with_menu(menu::create_tary_menu());

    let context = tauri::generate_context!();

    // 初始化窗口
    tauri::Builder::default()
        .invoke_handler(generate_handler![
            command::greet,
            command::get_window_mode_handler,
            command::set_window_mode_handler,
            command::is_enable_internal_script_handler,
            command::enable_internal_script_handler,
            command::set_preference_handler,
            command::get_preference_handler
        ])
        .menu(menu::create_window_menu())
        .plugin(tauri_plugin_positioner::init())
        .setup(|app| {
            let main_window = tauri::WindowBuilder::new(
                app,
                "main",
                tauri::WindowUrl::App("https://yiyan.baidu.com".into()),
            )
            .title(constant::APP_NAME)
            .enable_clipboard_access()
            .visible(false)
            .initialization_script(&plugin::load_internal_script("./plugin/base.js"))
            // .initialization_script(&plugin::load_internal_script("./plugin/third/html2canvas.js"))
            .initialization_script(&plugin::load_internal_script("./plugin/erniebot.js"))
            .initialization_script(&plugin::load_internal_script("./plugin/chatchat.js"))
            .build()
            .unwrap();

            let mut shortcut = app.global_shortcut_manager();
            shortcut
                .register("Cmd+Shift+O", move || {
                    if main_window.is_visible().unwrap() {
                        main_window.hide().unwrap();
                    } else {
                        main_window.show().unwrap();
                        main_window.set_focus().unwrap();
                    }
                })
                .unwrap_or_else(|err| println!("{:?}", err));

            if preference_util::get_window_mode() == WindowMode::Window {
                let main_window = app.get_window("main").unwrap();
                // main_window.eval(&plugin::load_internal_plugin()).unwrap();
                main_window
                    .set_size(PhysicalSize::new(
                        constant::WINDOW_WIDTH,
                        constant::WINDOW_HEIGHT,
                    ))
                    .unwrap();
                main_window.move_window(Position::Center).unwrap();
                main_window.set_decorations(true).unwrap();
                main_window.set_always_on_top(false).unwrap();
                main_window.set_skip_taskbar(false).unwrap();
                main_window.menu_handle().show().unwrap();
                main_window.show().unwrap();
                main_window.set_focus().unwrap();
            }
            Ok(())
        })
        // .menu(tauri::Menu::os_default(&context.package_info().name))
        .system_tray(tray)
        // 窗口监听
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
                api.prevent_close();
            }
            tauri::WindowEvent::Focused(is_focused) => {
                // 当点击外侧的时候隐藏窗口
                // 获取当前的窗口模式
                let mode = preference_util::get_window_mode();
                let auto_hide = preference_util::auto_hide_when_click_outside();
                if mode == WindowMode::TaskBar && auto_hide {
                    if !is_focused {
                        event.window().hide().unwrap();
                    }
                }
            }
            _ => {}
        })
        .on_menu_event(menu::on_window_event_handler)
        // 任务栏菜单监听
        .on_system_tray_event(menu::on_tray_event)
        .run(context)
        .expect("error while running tauri application");
}
