use log::info;
use tauri::{
    api, AppHandle, CustomMenuItem, GlobalWindowEvent, LogicalSize, Manager, Menu, MenuItem,
    PhysicalPosition, PhysicalSize, Submenu, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
    SystemTraySubmenu, Window, WindowMenuEvent,
};
use tauri_plugin_positioner::{Position, WindowExt};

use crate::{
    model::{
        constant::{self, PREFERENCE_CURRENT_PAGE_URL, WINDOW_LABEL_MAIN},
        extension_menu::ExtensionMenu,
        preference_model::WindowMode,
    },
    preference_util,
};

/// 创建右下角菜单
pub fn create_tary_menu() -> SystemTrayMenu {
    let open = CustomMenuItem::new("open".to_string(), "打开窗口").accelerator("Cmd+Shift+O");
    let quit = CustomMenuItem::new("quit".to_string(), "退出").accelerator("Cmd+Q");
    let github = CustomMenuItem::new("github".to_string(), "访问 Github");
    let gitee = CustomMenuItem::new("gitee".to_string(), "访问 Gitee");
    let feedback = CustomMenuItem::new("feedback".to_string(), "反馈问题");
    let preference = CustomMenuItem::new("preference".to_string(), "设置");
    let refresh = CustomMenuItem::new("refresh", "刷新");
    // let always_top = CustomMenuItem::new("always_top", "常驻置顶").selected();

    let mut internal_main_menu = SystemTrayMenu::new()
        .add_item(open)
        .add_item(refresh)
        .add_native_item(SystemTrayMenuItem::Separator);
    // .add_submenu(mode_submenu)
    // .add_submenu(image_submenu);

    // 生成菜单
    if let Some(list) = preference_util::get_internal_menu_list() {
        list.iter().for_each(|parent_menu| {
            if parent_menu.is_separator() {
                internal_main_menu = internal_main_menu
                    .clone()
                    .add_native_item(SystemTrayMenuItem::Separator);
            } else if !parent_menu.get_menu().is_empty() {
                internal_main_menu =
                    internal_main_menu
                        .clone()
                        .add_submenu(create_internal_tray_submenu(
                            &parent_menu.get_title(),
                            &parent_menu.get_menu(),
                        ))
            }
        });
    }

    if let Some(menu) = create_custom_tray_menu() {
        internal_main_menu = internal_main_menu.add_submenu(menu);
    }

    internal_main_menu = internal_main_menu
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(github)
        .add_item(gitee)
        .add_item(feedback)
        .add_item(preference)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    return internal_main_menu;
}

/// 创建自定义右下角菜单
fn create_custom_tray_menu() -> Option<SystemTraySubmenu> {
    if let Some(mut list) = preference_util::get_custom_menu_list() {
        if list.is_empty() {
            return None;
        }
        // info!("{:?}", &list);
        let mut menu = SystemTrayMenu::new();

        list.sort_by_key(|item| item.get_priority().unwrap_or_else(|| 0));
        list.iter().for_each(|item| {
            if item.get_name().is_some() && item.get_string_id().is_some() {
                menu = menu.clone().add_item(CustomMenuItem::new(
                    item.get_string_id().unwrap(),
                    item.get_name().unwrap(),
                ));
            }
        });
        // info!("{:?}", &menu);
        return Some(SystemTraySubmenu::new("我的平台", menu));
    }
    return None;
}

/// 创建自定义窗口菜单
fn create_custom_menu() -> Option<Submenu> {
    if let Some(mut list) = preference_util::get_custom_menu_list() {
        if list.is_empty() {
            return None;
        }
        info!("{:?}", &list);
        let mut menu = Menu::new();

        list.sort_by_key(|item| item.get_priority().unwrap_or_else(|| 0));
        list.iter().for_each(|item| {
            if item.get_name().is_some() && item.get_string_id().is_some() {
                menu = menu.clone().add_item(CustomMenuItem::new(
                    item.get_string_id().unwrap(),
                    item.get_name().unwrap(),
                ));
            }
        });
        // info!("{:?}", &menu);
        return Some(Submenu::new("我的平台", menu));
    }
    return None;
}

/// 创建窗口菜单
pub fn create_window_menu() -> Menu {
    // 创建普通菜单
    let github = CustomMenuItem::new("github".to_string(), "访问 Github");
    let gitee = CustomMenuItem::new("gitee".to_string(), "访问 Gitee");
    let feedback = CustomMenuItem::new("feedback", "反馈问题");

    // let about_meta_data = AboutMetadata::new().authors(vec!["Cloris".to_string()]);
    // let about = MenuItem::About("OneGPT".to_string(), about_meta_data);

    let about_submenu = Submenu::new(
        "帮助".to_string(),
        Menu::new()
            .add_item(github)
            .add_item(gitee)
            .add_item(feedback),
    );

    // 生成菜单
    let mut internal_main_menu = Menu::new();
    if let Some(list) = preference_util::get_internal_menu_list() {
        list.iter().for_each(|parent_menu| {
            if parent_menu.is_separator() {
                internal_main_menu = internal_main_menu
                    .clone()
                    .add_native_item(MenuItem::Separator);
            } else if !parent_menu.get_menu().is_empty() {
                internal_main_menu = internal_main_menu
                    .clone()
                    .add_submenu(create_window_submenu(
                        &parent_menu.get_title(),
                        &parent_menu.get_menu(),
                    ))
            }
        });
    }

    if let Some(submenu) = create_custom_menu() {
        internal_main_menu = internal_main_menu.add_submenu(submenu);
    }

    if cfg!(target_os = "macos") {
        let edit_menu = Submenu::new(
            "编辑",
            Menu::new()
                .add_native_item(MenuItem::Undo)
                .add_native_item(MenuItem::Redo)
                .add_native_item(MenuItem::Separator)
                .add_native_item(MenuItem::Cut)
                .add_native_item(MenuItem::Copy)
                .add_native_item(MenuItem::Paste)
                .add_native_item(MenuItem::SelectAll),
        );
        internal_main_menu = internal_main_menu.add_submenu(edit_menu);
    }

    internal_main_menu = internal_main_menu
        .add_item(CustomMenuItem::new("refresh", "刷新"))
        .add_item(CustomMenuItem::new("preference", "设置"))
        .add_submenu(about_submenu);
    return internal_main_menu;
}

/// 窗口菜单事件
pub fn on_window_event_handler(event: WindowMenuEvent) {
    // 内置菜单
    let mut binding = constant::MENU_MAP.try_lock().unwrap();
    let menu_map = binding.get_mut();
    if menu_map.contains_key(&event.menu_item_id().to_string()) {
        redirect_url(
            &event.window(),
            menu_map.get(&event.menu_item_id().to_string()).unwrap(),
        );
        return;
    }

    // 用户自定义菜单
    if let Some(extension_menu_list) = preference_util::get_custom_menu_list() {
        extension_menu_list.iter().for_each(|item| {
            if item.get_string_id().is_some()
                && (&event).menu_item_id() == item.get_string_id().unwrap()
                && item.get_url().is_some()
            {
                redirect_url(&event.window(), item.get_url().unwrap().as_str());
                return;
            }
        });
    }
    // 窗口菜单监听
    match event.menu_item_id() {
        "preference" => {
            show_window_to_center(
                &event
                    .window()
                    .get_window(constant::WINDOW_LABEL_PREFERENCE)
                    .unwrap(),
            );
        }
        "refresh" => {
            let url = preference_util::get_preference(PREFERENCE_CURRENT_PAGE_URL, "");
            event
                .window()
                .eval(&format!("window.location.replace('{}')", url))
                .unwrap();
        }
        "github" => {
            redirect_github(
                &event
                    .window()
                    .get_window(constant::WINDOW_LABEL_MAIN)
                    .unwrap(),
            );
        }
        "gitee" => {
            redirect_gitee(
                &event
                    .window()
                    .get_window(constant::WINDOW_LABEL_MAIN)
                    .unwrap(),
            );
        }

        "feedback" => {
            redirect_feedback(
                &event
                    .window()
                    .get_window(constant::WINDOW_LABEL_MAIN)
                    .unwrap(),
            );
        }

        // "wenxinyige" => {
        //     redirect_url(&event.window(), "https://yige.baidu.com/");
        // }
        _ => {}
    }
}

/// 任务栏事件
pub fn on_tray_event(app: &AppHandle, event: SystemTrayEvent) {
    tauri_plugin_positioner::on_tray_event(app, &event);
    let mini_mutex = constant::IS_MINIMIZED.try_lock().unwrap();
    match event {
        SystemTrayEvent::LeftClick { position, size, .. } => {
            let window = app.get_window("main").unwrap();
            let init_qq_mutex = constant::INIT_QQ_MODE.try_lock().unwrap();
            let init_window_mutex = constant::INIT_WINDOW_MODE.try_lock().unwrap();

            let mode = preference_util::get_window_mode();
            if mode == WindowMode::TaskBar {
                init_qq_mutex.set(false);
                init_window_mutex.set(false);
                // 任务栏模式
                window
                    .set_size(LogicalSize::new(
                        constant::TASK_WINDOW_WIDTH,
                        constant::TASK_WINDOW_HEIGHT,
                    ))
                    .unwrap();
                window.move_window(Position::TrayCenter).unwrap();
                window.set_decorations(false).unwrap();
                window.set_always_on_top(true).unwrap();
                window.set_skip_taskbar(true).unwrap();
                // window.menu_handle().hide().unwrap();
            } else if mode == WindowMode::SideBar {
                init_qq_mutex.set(false);
                init_window_mutex.set(false);
                // 侧边栏模式
                let screen = window.current_monitor().unwrap().unwrap();
                let screen_height = screen.size().height as i32;
                let screen_width = screen.size().width as i32;

                let side_bar_height = screen_height - size.height as i32 * 2;
                // let side_bar_y = position.y as i32 - side_bar_height;

                let physical_width =
                    constant::SIDE_BAR_WIDTH as f64 * window.scale_factor().unwrap();

                window
                    .set_size(PhysicalSize::new(physical_width as i32, side_bar_height))
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
            } else if mode == WindowMode::QQ {
                // QQ 模式
                info!("init_qq_mutex = {}", init_qq_mutex.get());

                if mini_mutex.get() {
                    window.unminimize().unwrap();
                }

                if !init_qq_mutex.get() {
                    let screen = window.current_monitor().unwrap().unwrap();
                    let screen_height = screen.size().height as i32;
                    let screen_width = screen.size().width as i32;

                    info!(
                        "outsize = {}, screen height = {}",
                        screen_height, screen_width
                    );

                    let physical_width =
                        constant::SIDE_BAR_WIDTH as f64 * window.scale_factor().unwrap();
                    window
                        .set_size(PhysicalSize::new(
                            physical_width as i32,
                            screen_height - 500,
                        ))
                        .unwrap();

                    window
                        .set_position(PhysicalPosition::new(
                            screen_width - window.outer_size().unwrap().width as i32 - 100,
                            (screen_height - window.outer_size().unwrap().height as i32) / 2,
                        ))
                        .unwrap();
                    init_qq_mutex.set(true);
                    // window.move_window(Position::TrayRight).unwrap();
                }
                window.set_decorations(true).unwrap();
                window.set_always_on_top(true).unwrap();
                window.set_skip_taskbar(true).unwrap();
                window.menu_handle().show().unwrap();
            } else {
                // 窗口模式
                init_qq_mutex.set(false);

                if !init_window_mutex.get() {
                    window
                        .set_size(LogicalSize::new(
                            constant::WINDOW_WIDTH,
                            constant::WINDOW_HEIGHT,
                        ))
                        .unwrap();
                    window.move_window(Position::Center).unwrap();
                    init_window_mutex.set(true);
                }

                window.set_decorations(true).unwrap();
                window.set_always_on_top(false).unwrap();
                window.set_skip_taskbar(false).unwrap();
                window.menu_handle().show().unwrap();
            }

            if mini_mutex.get() && mode == WindowMode::QQ {
                window.show().unwrap();
                window.set_focus().unwrap();
            } else if mini_mutex.get() && mode == WindowMode::Window {
                window.unminimize().unwrap();
                window.show().unwrap();
                window.set_focus().unwrap();
            } else {
                if window.is_visible().unwrap() {
                    window.hide().unwrap();
                } else {
                    window.show().unwrap();
                    window.set_focus().unwrap();
                }
            }

            // info!("window.is_visible = {:?}", &window.is_visible());
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
        SystemTrayEvent::MenuItemClick { id, .. } => {
            // 内置菜单
            let mut binding = constant::MENU_MAP.try_lock().unwrap();
            let menu_map = binding.get_mut();
            if menu_map.contains_key(&id) {
                redirect_url(
                    &app.get_window(constant::WINDOW_LABEL_MAIN).unwrap(),
                    menu_map.get(&id).unwrap(),
                );
                return;
            }

            // 用户自定义菜单
            if let Some(extension_menu_list) = preference_util::get_custom_menu_list() {
                extension_menu_list.iter().for_each(|item| {
                    if item.get_string_id().is_some()
                        && id.as_str() == item.get_string_id().unwrap()
                        && item.get_url().is_some()
                    {
                        redirect_url(
                            &app.get_window(constant::WINDOW_LABEL_MAIN).unwrap(),
                            item.get_url().unwrap().as_str(),
                        );
                    }
                });
            }
            match id.as_str() {
                "github" => {
                    redirect_github(&app.get_window(constant::WINDOW_LABEL_MAIN).unwrap());
                }
                "gitee" => {
                    redirect_gitee(&app.get_window(constant::WINDOW_LABEL_MAIN).unwrap());
                }
                "feedback" => {
                    redirect_feedback(&app.get_window(constant::WINDOW_LABEL_MAIN).unwrap());
                }
                "refresh" => {
                    let url = preference_util::get_preference(PREFERENCE_CURRENT_PAGE_URL, "");
                    app.get_window("main")
                        .unwrap()
                        .eval(&format!("window.location.replace('{}')", url))
                        .unwrap();
                }
                // "bard" => {
                //     redirect_url(&app.get_window("main").unwrap(), "https://bard.google.com/");
                // }
                "quit" => {
                    std::process::exit(0);
                }
                "preference" => {
                    show_window_to_center(
                        &app.get_window(constant::WINDOW_LABEL_PREFERENCE).unwrap(),
                    );
                }
                "open" => {
                    let main_window = app.get_window("main").unwrap();
                    main_window.show().unwrap();
                    main_window.set_focus().unwrap();
                }
                _ => {}
            }
        }
        _ => {}
    }
}

/// 窗口监听
pub fn on_window_event(event: GlobalWindowEvent) {
    match event.event() {
        tauri::WindowEvent::CloseRequested { api, .. } => {
            // info!("{}", event.window().label());
            // 非Main窗口都隐藏
            if event.window().label() != WINDOW_LABEL_MAIN {
                event.window().hide().unwrap();
                api.prevent_close();
                return;
            }

            if !preference_util::is_exit_app() {
                event.window().hide().unwrap();
                api.prevent_close();
            } else {
                std::process::exit(0);
            }
        }
        tauri::WindowEvent::Focused(is_focused) => {
            // 当点击外侧的时候隐藏窗口
            // 获取当前的窗口模式
            let mode = preference_util::get_window_mode();
            let auto_hide = preference_util::auto_hide_when_click_outside();
            if mode == WindowMode::TaskBar
                && auto_hide
                && event.window().label() == constant::WINDOW_LABEL_MAIN
            {
                if !is_focused {
                    event.window().hide().unwrap();
                }
            }
        }
        tauri::WindowEvent::Resized(physical_size) => {
            // info!("physical_size: {:?}", physical_size);
            // let size_mutex = constant::BRFORE_WINDOW_MINIMIZED_SIZE.try_lock().unwrap();
            if event.window().label() != WINDOW_LABEL_MAIN {
                return;
            }
            let mini_mutex = constant::IS_MINIMIZED.try_lock().unwrap();
            if physical_size.height == 0 && physical_size.width == 0 {
                // 窗口被最小化
                // info!("MINIMIZED");
                mini_mutex.set(true);
            } else {
                // info!("NO MINIMIZED");
                mini_mutex.set(false);
                // size_mutex.set((physical_size.width, physical_size.height));
            }
            // info!("minimize = {}", mini_mutex.get());
        }
        // tauri::WindowEvent::Moved(physical_position) => {
        // let position_mutex = constant::BRFORE_WINDOW_MINIMIZED_POSITION
        //     .try_lock()
        //     .unwrap();
        // info!("physical_position: {:?}", physical_position);
        // constant::BRFORE_WINDOW_MINIMIZED_SIZE.set((physical_position.width, physical_size.height));
        // if physical_position.x > -30000 && physical_position.y > -30000 {
        //     position_mutex.set((physical_position.x, physical_position.y));
        //     // info!("after physical_position: {:?}", &position_mutex);
        // }
        // }
        _ => {}
    }
}

/// 跳转到github
fn redirect_github(window: &Window) {
    api::shell::open(
        &window.shell_scope(),
        "https://github.com/1595901624/gpt-aggregated-edition".to_string(),
        None,
    )
    .unwrap();
}

/// 跳转到gitee
fn redirect_gitee(window: &Window) {
    api::shell::open(
        &window.shell_scope(),
        "https://gitee.com/haoyu3/gpt-aggregated-edition.git".to_string(),
        None,
    )
    .unwrap();
}

/// 跳转到 Github issue 页面
fn redirect_feedback(window: &Window) {
    api::shell::open(
        &window.shell_scope(),
        "https://github.com/1595901624/gpt-aggregated-edition/issues",
        None,
    )
    .unwrap()
}

/// 跳转到某个页面
fn redirect_url(window: &Window, url: &str) {
    // let cur = preference_util::get_preference(constant::PREFERENCE_CURRENT_PAGE_URL, "");
    // if cur.to_string() == url {
    //     return;
    // }
    window
        .eval(&format!("window.location.replace('{}')", url))
        .unwrap();
    if !window.is_visible().unwrap() {
        window.show().unwrap();
        window.set_focus().unwrap();
    }
    preference_util::set_preference(constant::PREFERENCE_CURRENT_PAGE_URL, url);
}

/// 通用的窗口显示
/// 将窗口显示到屏幕中心
fn show_window_to_center(window: &Window) {
    window.move_window(Position::Center).unwrap();
    // window.menu_handle().hide().unwrap();
    window.show().unwrap();
    window.set_focus().unwrap();
}

/// 递归创建窗口子菜单
fn create_window_submenu(name: &String, extension_menu_list: &Vec<ExtensionMenu>) -> Submenu {
    let mut menu = Menu::new();
    extension_menu_list.iter().for_each(|item| {
        if item.exist_submenu() {
            menu = menu.clone().add_submenu(create_window_submenu(
                &item.get_name().unwrap(),
                &item.get_submenu(),
            ));
        } else if item.is_separator() {
            menu = menu.clone().add_native_item(MenuItem::Separator);
        } else {
            menu = menu.clone().add_item(CustomMenuItem::new(
                item.get_string_id().unwrap(),
                item.get_name().unwrap(),
            ));
            constant::MENU_MAP
                .try_lock()
                .unwrap()
                .get_mut()
                .insert(item.get_string_id().unwrap(), item.get_url().unwrap());
        }
    });
    return Submenu::new(name, menu);
}

/// 递归创建任务栏子菜单
fn create_internal_tray_submenu(
    name: &String,
    extension_menu_list: &Vec<ExtensionMenu>,
) -> SystemTraySubmenu {
    let mut menu = SystemTrayMenu::new();
    extension_menu_list.iter().for_each(|item| {
        if item.exist_submenu() {
            menu = menu.clone().add_submenu(create_internal_tray_submenu(
                &item.get_name().unwrap(),
                &item.get_submenu(),
            ));
        } else if item.is_separator() {
            menu = menu.clone().add_native_item(SystemTrayMenuItem::Separator);
        } else {
            menu = menu.clone().add_item(CustomMenuItem::new(
                item.get_string_id().unwrap(),
                item.get_name().unwrap(),
            ));
        }
    });
    return SystemTraySubmenu::new(name, menu);
}
