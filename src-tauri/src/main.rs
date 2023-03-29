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

use std::{thread::sleep, time::Duration};

use model::preference_model::WindowMode;
use tauri::{
    api, generate_handler, CustomMenuItem, GlobalShortcutManager, Manager, Menu, PhysicalPosition,
    PhysicalSize, Submenu, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};
use tauri_plugin_positioner::{Position, WindowExt};

fn main() {
    preference_util::init_default_preference();

    // 创建右下角菜单
    let open = CustomMenuItem::new("open".to_string(), "打开窗口")
        .accelerator("Cmd+Shift+O");
    let quit = CustomMenuItem::new("quit".to_string(), "退出").accelerator("Cmd+Q");
    let chat_gpt = CustomMenuItem::new("chat_gpt".to_string(), "ChatGPT(免费版)");
    let chat_chat = CustomMenuItem::new("chat_chat".to_string(), "ChatGPT(限额版)");
    let chat_gpt_official = CustomMenuItem::new("chat_gpt_official".to_string(), "ChatGPT(官方版)");
    let ernie_bot = CustomMenuItem::new("ernie_bot".to_string(), "文心一言");
    let poe = CustomMenuItem::new("poe".to_string(), "POE");
    let bard = CustomMenuItem::new("bard".to_string(), "Bard");
    let github = CustomMenuItem::new("github".to_string(), "访问 Github");
    let gitee = CustomMenuItem::new("gitee".to_string(), "访问 Gitee");
    let preference = CustomMenuItem::new("preference".to_string(), "设置");
    let refresh = CustomMenuItem::new("refresh", "刷新");
    // let always_top = CustomMenuItem::new("always_top", "常驻置顶").selected();
    let tray_menu = SystemTrayMenu::new()
        .add_item(open)
        .add_item(refresh)
        // .add_item(always_top)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(ernie_bot)
        .add_item(chat_chat)
        .add_item(chat_gpt)
        .add_item(chat_gpt_official)
        .add_item(poe)
        .add_item(bard)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(github)
        .add_item(gitee)
        .add_item(preference)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    let tray = SystemTray::new().with_menu(tray_menu);

    let context = tauri::generate_context!();

    // 创建普通菜单
    let chat_gpt = CustomMenuItem::new("chat_gpt".to_string(), "ChatGPT(免费版)");
    let chat_gpt_official = CustomMenuItem::new("chat_gpt_official".to_string(), "ChatGPT(官方版)");
    let ernie_bot = CustomMenuItem::new("ernie_bot".to_string(), "文心一言");
    let chat_chat = CustomMenuItem::new("chat_chat".to_string(), "ChatGPT(限额版)");
    let poe = CustomMenuItem::new("poe".to_string(), "POE");
    let bard = CustomMenuItem::new("bard".to_string(), "Bard");
    let github = CustomMenuItem::new("github".to_string(), "访问 Github");
    let gitee = CustomMenuItem::new("gitee".to_string(), "访问 Gitee");
    // let close = CustomMenuItem::new("close".to_string(), "Close");
    let mode_submenu = Submenu::new(
        "平台切换",
        Menu::new()
            .add_item(ernie_bot)
            .add_item(chat_chat)
            .add_item(chat_gpt)
            .add_item(chat_gpt_official)
            .add_item(poe)
            .add_item(bard),
    );
    let about_submenu = Submenu::new(
        "更多".to_string(),
        Menu::new().add_item(github).add_item(gitee),
    );

    let menu = Menu::new()
        .add_submenu(mode_submenu)
        .add_item(CustomMenuItem::new("refresh", "刷新"))
        .add_item(CustomMenuItem::new("preference", "设置"))
        .add_submenu(about_submenu);

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
        .menu(menu)
        .plugin(tauri_plugin_positioner::init())
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();
            main_window.eval(&plugin::load_system_js()).unwrap();
            main_window.eval(&plugin::load_internal_plugin()).unwrap();

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

            // if preference_util::get_window_mode() == WindowMode::Window {
            //     let main_window = app.get_window("main").unwrap();
            //     main_window.eval(&plugin::load_internal_plugin()).unwrap();
            //     main_window
            //         .set_size(PhysicalSize::new(
            //             constant::WINDOW_WIDTH,
            //             constant::WINDOW_HEIGHT,
            //         ))
            //         .unwrap();
            //     main_window.move_window(Position::Center).unwrap();
            //     main_window.set_decorations(true).unwrap();
            //     main_window.set_always_on_top(false).unwrap();
            //     main_window.set_skip_taskbar(false).unwrap();
            //     main_window.menu_handle().show().unwrap();
            //     main_window.show().unwrap();
            //     main_window.set_focus().unwrap();
            // }
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
                // 当点击外测的时候隐藏窗口
                // 获取当前的窗口模式
                let mode = preference_util::get_window_mode();
                if mode == WindowMode::TaskBar {
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
                    // event.window().get_window("main").unwrap().o
                    // let main_window = app.get_window("main").unwrap();
                    // main_window.show().unwrap();
                    // main_window.set_focus().unwrap();
                    // main_window.eval(&format!(
                    //     "window.location.replace('https://sonnylab-gpt.vercel.app')"
                    // ));
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
                _ => {}
            }
            sleep(Duration::from_millis(constant::SWITCH_PAGE_SLEEP_TIME));
            //event.window().eval(&plugin::load_system_js()).unwrap();
            event
                .window()
                .eval(&plugin::load_internal_plugin())
                .unwrap();
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

                        dbg!(&position);
                        dbg!(&size);

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

                        //window.eval(&plugin::load_system_js()).unwrap();
                        window
                            .eval(&plugin::load_internal_plugin())
                            .map_err(|err| println!("{:?}", err))
                            .ok();
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
                        sleep(Duration::from_millis(constant::SWITCH_PAGE_SLEEP_TIME));
                        //main_window.eval(&plugin::load_system_js()).unwrap();
                        main_window.eval(&plugin::load_internal_plugin()).unwrap();
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
                        sleep(Duration::from_millis(constant::SWITCH_PAGE_SLEEP_TIME));
                        main_window.eval(&plugin::load_system_js()).unwrap();
                        main_window.eval(&plugin::load_internal_plugin()).unwrap();
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
                        sleep(Duration::from_millis(constant::SWITCH_PAGE_SLEEP_TIME));
                        main_window.eval(&plugin::load_system_js()).unwrap();
                        main_window.eval(&plugin::load_internal_plugin()).unwrap();
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
                        sleep(Duration::from_millis(constant::SWITCH_PAGE_SLEEP_TIME));
                        main_window.eval(&plugin::load_system_js()).unwrap();
                        main_window.eval(&plugin::load_internal_plugin()).unwrap();
                    }
                    "poe" => {
                        let main_window = app.get_window("main").unwrap();
                        main_window.show().unwrap();
                        main_window.set_focus().unwrap();
                        main_window
                            .eval(&format!("window.location.replace('https://poe.com/')"))
                            .unwrap();
                        sleep(Duration::from_millis(constant::SWITCH_PAGE_SLEEP_TIME));
                        main_window.eval(&plugin::load_system_js()).unwrap();
                        main_window.eval(&plugin::load_internal_plugin()).unwrap();
                    }
                    "refresh" => {
                        let main_window = app.get_window("main").unwrap();
                        main_window
                            .eval(&format!("window.location.replace(window.location.href)"))
                            .unwrap();
                        sleep(Duration::from_millis(constant::SWITCH_PAGE_SLEEP_TIME));
                        main_window.eval(&plugin::load_system_js()).unwrap();
                        main_window.eval(&plugin::load_internal_plugin()).unwrap();
                    }
                    "bard" => {
                        let main_window = app.get_window("main").unwrap();
                        main_window
                            .eval(&format!(
                                "window.location.replace('https://bard.google.com/')"
                            ))
                            .unwrap();
                        sleep(Duration::from_millis(constant::SWITCH_PAGE_SLEEP_TIME));
                        main_window.eval(&plugin::load_system_js()).unwrap();
                        main_window.eval(&plugin::load_internal_plugin()).unwrap();
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
                        main_window.eval(&plugin::load_internal_plugin()).unwrap();
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
