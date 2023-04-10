use tauri::{
    api, AppHandle, CustomMenuItem, Manager, Menu, PhysicalPosition, PhysicalSize, Submenu,
    SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem, SystemTraySubmenu, Window,
    WindowMenuEvent,
};
use tauri_plugin_positioner::{Position, WindowExt};

use crate::{
    model::{
        constant::{self, PREFERENCE_CURRENT_PAGE_URL},
        preference_model::WindowMode,
    },
    preference_util,
};

/// 创建右下角菜单
pub fn create_tary_menu() -> SystemTrayMenu {
    let open = CustomMenuItem::new("open".to_string(), "打开窗口").accelerator("Cmd+Shift+O");
    let quit = CustomMenuItem::new("quit".to_string(), "退出").accelerator("Cmd+Q");
    let chat_gpt = CustomMenuItem::new("chat_gpt".to_string(), "ChatGPT(免费线路1)");
    let chat_gpt_free2 = CustomMenuItem::new("chat_gpt_free2".to_string(), "ChatGPT(免费线路2)");
    let chat_gpt_free3 = CustomMenuItem::new("chat_gpt_free3".to_string(), "ChatGPT(免费线路3)");
    let chat_chat = CustomMenuItem::new("chat_chat".to_string(), "ChatGPT(限额版)");
    let chat_gpt_official = CustomMenuItem::new("chat_gpt_official".to_string(), "ChatGPT(官方版)");
    let ernie_bot = CustomMenuItem::new("ernie_bot".to_string(), "文心一言");
    let tongyi = CustomMenuItem::new("tongyi".to_string(), "通义千问");
    let poe = CustomMenuItem::new("poe".to_string(), "POE");
    let bard = CustomMenuItem::new("bard".to_string(), "Bard");
    let bing = CustomMenuItem::new("bing".to_string(), "NewBing");
    let github = CustomMenuItem::new("github".to_string(), "访问 Github");
    let gitee = CustomMenuItem::new("gitee".to_string(), "访问 Gitee");
    let preference = CustomMenuItem::new("preference".to_string(), "设置");
    let refresh = CustomMenuItem::new("refresh", "刷新");
    // let always_top = CustomMenuItem::new("always_top", "常驻置顶").selected();

    let mode_submenu = SystemTraySubmenu::new(
        "AI对话平台",
        SystemTrayMenu::new()
            .add_item(ernie_bot)
            .add_item(tongyi)
            .add_item(chat_chat)
            .add_item(chat_gpt)
            .add_item(chat_gpt_free2)
            .add_item(chat_gpt_free3)
            .add_item(chat_gpt_official)
            .add_item(bing)
            .add_item(poe)
            .add_item(bard),
    );
    // AI图像平台
    let wenxinyige = CustomMenuItem::new("wenxinyige".to_string(), "文心一格");
    let image_submenu =
        SystemTraySubmenu::new("AI图像平台", SystemTrayMenu::new().add_item(wenxinyige));
    SystemTrayMenu::new()
        .add_item(open)
        .add_item(refresh)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_submenu(mode_submenu)
        .add_submenu(image_submenu)
        // .add_item(always_top)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(github)
        .add_item(gitee)
        .add_item(preference)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit)
}

/// 创建窗口菜单
pub fn create_window_menu() -> Menu {
    // 创建普通菜单
    let chat_gpt = CustomMenuItem::new("chat_gpt".to_string(), "ChatGPT(免费线路1)");
    let chat_gpt_free2 = CustomMenuItem::new("chat_gpt_free2".to_string(), "ChatGPT(免费线路2)");
    let chat_gpt_free3 = CustomMenuItem::new("chat_gpt_free3".to_string(), "ChatGPT(免费线路3)");
    let chat_gpt_official = CustomMenuItem::new("chat_gpt_official".to_string(), "ChatGPT(官方版)");
    let ernie_bot = CustomMenuItem::new("ernie_bot".to_string(), "文心一言");
    let tongyi = CustomMenuItem::new("tongyi".to_string(), "通义千问");
    let chat_chat = CustomMenuItem::new("chat_chat".to_string(), "ChatGPT(限额版)");
    let poe = CustomMenuItem::new("poe".to_string(), "POE");
    let bing = CustomMenuItem::new("bing".to_string(), "NewBing");
    let bard = CustomMenuItem::new("bard".to_string(), "Bard");
    let github = CustomMenuItem::new("github".to_string(), "访问 Github");
    let gitee = CustomMenuItem::new("gitee".to_string(), "访问 Gitee");
    // let close = CustomMenuItem::new("close".to_string(), "Close");
    // AI对话平台
    let mode_submenu = Submenu::new(
        "AI对话平台",
        Menu::new()
            .add_item(ernie_bot)
            .add_item(tongyi)
            .add_item(chat_chat)
            .add_item(chat_gpt)
            .add_item(chat_gpt_free2)
            .add_item(chat_gpt_free3)
            .add_item(chat_gpt_official)
            .add_item(bing)
            .add_item(poe)
            .add_item(bard),
    );

    // AI图像平台
    let wenxinyige = CustomMenuItem::new("wenxinyige".to_string(), "文心一格");
    let image_submenu = Submenu::new("AI图像平台", Menu::new().add_item(wenxinyige));
    let about_submenu = Submenu::new(
        "更多".to_string(),
        Menu::new().add_item(github).add_item(gitee),
    );

    Menu::new()
        .add_submenu(mode_submenu)
        .add_submenu(image_submenu)
        .add_item(CustomMenuItem::new("refresh", "刷新"))
        .add_item(CustomMenuItem::new("preference", "设置"))
        .add_submenu(about_submenu)
}

/// 窗口菜单事件
pub fn on_window_event_handler(event: WindowMenuEvent) {
    // 窗口菜单监听
    match event.menu_item_id() {
        "ernie_bot" => {
            redirect_url(&event.window(), "https://yiyan.baidu.com/");
        }
        "tongyi" => {
            redirect_url(&event.window(), "https://tongyi.aliyun.com/");
        }
        "chat_chat" => {
            redirect_url(&event.window(), "https://chat.okis.dev/zh-CN?mode=chat");
        }
        "chat_gpt" => {
            redirect_url(&event.window(), "https://freegpt.one/");
        }
        "chat_gpt_free2" => {
            redirect_url(&event.window(), "https://chatbot.theb.ai/");
        }
        "chat_gpt_free3" => {
            redirect_url(&event.window(), "https://chatgpt-35-turbo.com/");
        }
        "chat_gpt_official" => {
            redirect_url(&event.window(), "https://chat.openai.com/chat");
        }
        "poe" => {
            redirect_url(&event.window(), "https://poe.com/");
        }
        "bing" => {
            redirect_url(&event.window(), "https://www.bing.com/new");
        }
        "bard" => {
            redirect_url(&event.window(), "https://bard.google.com/");
        }
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

        "wenxinyige" => {
            redirect_url(&event.window(), "https://yige.baidu.com/");
        }
        _ => {}
    }
}

/// 任务栏事件
pub fn on_tray_event(app: &AppHandle, event: SystemTrayEvent) {
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
                redirect_github(&app.get_window(constant::WINDOW_LABEL_MAIN).unwrap());
            }
            "gitee" => {
                redirect_gitee(&app.get_window(constant::WINDOW_LABEL_MAIN).unwrap());
            }
            "ernie_bot" => {
                redirect_url(&app.get_window("main").unwrap(), "https://yiyan.baidu.com/");
            }
            "tongyi" => {
                redirect_url(&app.get_window("main").unwrap(), "https://tongyi.aliyun.com/");
            }
            "chat_chat" => {
                redirect_url(
                    &app.get_window("main").unwrap(),
                    "https://chat.okis.dev/zh-CN?mode=chat",
                );
            }
            "chat_gpt" => {
                redirect_url(&app.get_window("main").unwrap(), "https://freegpt.one/");
                // main_window.eval(&format!(
                //     "window.location.replace('https://sonnylab-gpt.vercel.app')"
                // ));
            }
            "chat_gpt_free2" => {
                redirect_url(&app.get_window("main").unwrap(), "https://chatbot.theb.ai/");
            }
            "chat_gpt_free3" => {
                redirect_url(
                    &app.get_window("main").unwrap(),
                    "https://chatgpt-35-turbo.com/",
                );
            }
            "chat_gpt_official" => {
                redirect_url(
                    &app.get_window("main").unwrap(),
                    "https://chat.openai.com/chat",
                );
            }
            "poe" => {
                redirect_url(&app.get_window("main").unwrap(), "https://poe.com/");
            }
            "bing" => {
                redirect_url(&app.get_window("main").unwrap(), "https://www.bing.com/new");
            }
            "refresh" => {
                let url = preference_util::get_preference(PREFERENCE_CURRENT_PAGE_URL, "");
                app.get_window("main")
                    .unwrap()
                    .eval(&format!("window.location.replace('{}')", url))
                    .unwrap();
            }
            "bard" => {
                redirect_url(&app.get_window("main").unwrap(), "https://bard.google.com/");
            }
            "quit" => {
                std::process::exit(0);
            }
            "preference" => {
                show_window_to_center(&app.get_window(constant::WINDOW_LABEL_PREFERENCE).unwrap());
            }
            "open" => {
                let main_window = app.get_window("main").unwrap();
                main_window.show().unwrap();
                main_window.set_focus().unwrap();
            }
            "wenxinyige" => {
                redirect_url(&app.get_window("main").unwrap(), "https://yige.baidu.com/");
            }
            _ => {}
        },
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
    window.menu_handle().hide().unwrap();
    window.show().unwrap();
    window.set_focus().unwrap();
}
