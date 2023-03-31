// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod command;
mod constant;
mod model;
mod plugin;
mod preference_util;
mod menu;

use model::preference_model::WindowMode;
use tauri::{
    api, generate_handler, GlobalShortcutManager, Manager, PhysicalPosition,
    PhysicalSize, SystemTray, SystemTrayEvent,
};
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
            .initialization_script(&plugin::load_internal_script("./plugin/base.js"))
            .initialization_script(&plugin::load_internal_script("./plugin/erniebot.js"))
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
        // 窗口菜单监听
        .on_menu_event(|event| {
            match event.menu_item_id() {
                "ernie_bot" => {
                    event.window().set_focus().unwrap();
                    event
                        .window()
                        .eval(&format!(
                            "window.location.replace('https://yiyan.baidu.com/')",
                        ))
                        .unwrap();
                }
                "chat_chat" => {
                    event.window().set_focus().unwrap();
                    event
                        .window()
                        .eval(&format!(
                            "window.location.replace('https://chat.okis.dev/zh-CN?mode=chat')"
                        ))
                        .unwrap();
                }
                "chat_gpt" => {
                    event
                        .window()
                        .eval("window.location.replace('https://freegpt.one/')")
                        .unwrap();
                }
                "chat_gpt_official" => {
                    event
                        .window()
                        .eval(&format!(
                            "window.location.replace('https://chat.openai.com/chat')"
                        ))
                        .unwrap();
                }
                "poe" => {
                    event
                        .window()
                        .eval(&format!("window.location.replace('https://poe.com/')"))
                        .unwrap();
                }
                "bing" => {
                    event
                        .window()
                        .eval(&format!(
                            "window.location.replace('https://www.bing.com/new')"
                        ))
                        .unwrap();
                }
                "bard" => {
                    event
                        .window()
                        .eval(&format!(
                            "window.location.replace('https://bard.google.com/')"
                        ))
                        .unwrap();
                }
                "preference" => {
                    let preference_window = event.window().get_window("preference").unwrap();
                    preference_window.move_window(Position::Center).unwrap();
                    preference_window.menu_handle().hide().unwrap();
                    preference_window.show().unwrap();
                    preference_window.set_focus().unwrap();
                }
                "refresh" => {
                    event
                        .window()
                        .eval(&format!("window.location.replace(window.location.href)"))
                        .unwrap();
                }
                "github" => {
                    api::shell::open(
                        &event.window().shell_scope(),
                        "https://github.com/1595901624/gpt-aggregated-edition".to_string(),
                        None,
                    )
                    .unwrap();
                }
                "gitee" => {
                    api::shell::open(
                        &event.window().shell_scope(),
                        "https://gitee.com/haoyu3/gpt-aggregated-edition.git".to_string(),
                        None,
                    )
                    .unwrap();
                }
                _ => {}
            }
        })
        // 任务栏菜单监听
        .on_system_tray_event(|app, event| {
            tauri_plugin_positioner::on_tray_event(app, &event);
            match event {
                SystemTrayEvent::LeftClick { position, size, .. } => {
                    let window = app.get_window("main").unwrap();

                    let mode = preference_util::get_window_mode();
                    if mode == WindowMode::TaskBar {
                        // 任务栏模式
                        window
                            .set_size(PhysicalSize::new(
                                constant::TASK_WINDOW_WIDTH,
                                constant::TASK_WINDOW_HEIGHT,
                            ))
                            .unwrap();
                        window.move_window(Position::TrayCenter).unwrap();
                        window.set_decorations(false).unwrap();
                        window.set_always_on_top(true).unwrap();
                        window.set_skip_taskbar(true).unwrap();
                        window.menu_handle().hide().unwrap();
                    } else if mode == WindowMode::SideBar {
                        // 侧边栏模式
                        let screen = window.current_monitor().unwrap().unwrap();
                        let screen_height = screen.size().height as i32;
                        let screen_width = screen.size().width as i32;

                        let side_bar_height = screen_height - size.height as i32 * 2;
                        // let side_bar_y = position.y as i32 - side_bar_height;

                        window
                            .set_size(PhysicalSize::new(constant::SIDE_BAR_WIDTH, side_bar_height))
                            .unwrap();
                        window
                            .set_position(PhysicalPosition::new(
                                screen_width - window.outer_size().unwrap().width as i32,
                                position.y as i32 - window.outer_size().unwrap().height as i32,
                            ))
                            .unwrap();
                        // window.move_window(Position::TrayRight).unwrap();
                        window.set_decorations(false).unwrap();
                        window.set_always_on_top(true).unwrap();
                        window.set_skip_taskbar(true).unwrap();
                        window.menu_handle().show().unwrap();
                    } else {
                        // 桌面模式
                        window
                            .set_size(PhysicalSize::new(
                                constant::WINDOW_WIDTH,
                                constant::WINDOW_HEIGHT,
                            ))
                            .unwrap();
                        window.move_window(Position::Center).unwrap();
                        window.set_decorations(true).unwrap();
                        window.set_always_on_top(false).unwrap();
                        window.set_skip_taskbar(false).unwrap();
                        window.menu_handle().show().unwrap();
                    }

                    if window.is_visible().unwrap() {
                        window.hide().unwrap();
                    } else {
                        window.show().unwrap();
                        window.set_focus().unwrap();
                    }
                    // app.get_window("main").unwrap().show().unwrap();
                    // app.get_window("main").unwrap().set_focus().unwrap();
                }
                SystemTrayEvent::RightClick {
                    position: _,
                    size: _,
                    ..
                } => {
                    //println!("system tray received a right click");
                }
                SystemTrayEvent::DoubleClick {
                    position: _,
                    size: _,
                    ..
                } => {
                    //println!("system tray received a double click");
                }
                SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                    "github" => {
                        api::shell::open(
                            &app.get_window("main").unwrap().shell_scope(),
                            "https://github.com/1595901624/gpt-aggregated-edition".to_string(),
                            None,
                        )
                        .unwrap();
                    }
                    "gitee" => {
                        api::shell::open(
                            &app.get_window("main").unwrap().shell_scope(),
                            "https://gitee.com/haoyu3/gpt-aggregated-edition.git".to_string(),
                            None,
                        )
                        .unwrap();
                    }
                    "ernie_bot" => {
                        let main_window = app.get_window("main").unwrap();
                        main_window.show().unwrap();
                        main_window.set_focus().unwrap();
                        main_window
                            .eval(&format!(
                                "window.location.replace('https://yiyan.baidu.com/')"
                            ))
                            .unwrap();
                    }
                    "chat_chat" => {
                        let main_window = app.get_window("main").unwrap();
                        main_window.show().unwrap();
                        main_window.set_focus().unwrap();
                        main_window
                            .eval(&format!(
                                "window.location.replace('https://chat.okis.dev/zh-CN?mode=chat')"
                            ))
                            .unwrap();
                    }
                    "chat_gpt" => {
                        let main_window = app.get_window("main").unwrap();
                        main_window.show().unwrap();
                        main_window.set_focus().unwrap();
                        main_window
                            .eval("window.location.replace('https://freegpt.one/')")
                            .unwrap();
                        // main_window.eval(&format!(
                        //     "window.location.replace('https://sonnylab-gpt.vercel.app')"
                        // ));
                    }
                    "chat_gpt_official" => {
                        let main_window = app.get_window("main").unwrap();
                        main_window.show().unwrap();
                        main_window.set_focus().unwrap();
                        main_window
                            .eval(&format!(
                                "window.location.replace('https://chat.openai.com/chat')"
                            ))
                            .unwrap();
                    }
                    "poe" => {
                        let main_window = app.get_window("main").unwrap();
                        main_window.show().unwrap();
                        main_window.set_focus().unwrap();
                        main_window
                            .eval(&format!("window.location.replace('https://poe.com/')"))
                            .unwrap();
                    }
                    "bing" => {
                        let main_window = app.get_window("main").unwrap();
                        main_window
                            .eval(&format!(
                                "window.location.replace('https://www.bing.com/new')"
                            ))
                            .unwrap();
                    }
                    "refresh" => {
                        let main_window = app.get_window("main").unwrap();
                        main_window
                            .eval(&format!("window.location.replace(window.location.href)"))
                            .unwrap();
                    }
                    "bard" => {
                        let main_window = app.get_window("main").unwrap();
                        main_window
                            .eval(&format!(
                                "window.location.replace('https://bard.google.com/')"
                            ))
                            .unwrap();
                    }
                    "quit" => {
                        std::process::exit(0);
                    }
                    "preference" => {
                        let preference_window = app.get_window("preference").unwrap();
                        preference_window.move_window(Position::Center).unwrap();
                        preference_window.menu_handle().hide().unwrap();
                        preference_window.show().unwrap();
                        preference_window.set_focus().unwrap();
                    }
                    "open" => {
                        let main_window = app.get_window("main").unwrap();
                        main_window.show().unwrap();
                        main_window.set_focus().unwrap();
                    }
                    _ => {}
                },
                _ => {}
            }
        })
        .run(context)
        .expect("error while running tauri application");
}
