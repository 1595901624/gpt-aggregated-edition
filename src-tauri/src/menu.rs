use tauri::{
    api, CustomMenuItem, Manager, Menu, Submenu, SystemTrayMenu, SystemTrayMenuItem,
    WindowMenuEvent,
};
use tauri_plugin_positioner::{Position, WindowExt};

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
    let poe = CustomMenuItem::new("poe".to_string(), "POE");
    let bard = CustomMenuItem::new("bard".to_string(), "Bard");
    let bing = CustomMenuItem::new("bing".to_string(), "NewBing");
    let github = CustomMenuItem::new("github".to_string(), "访问 Github");
    let gitee = CustomMenuItem::new("gitee".to_string(), "访问 Gitee");
    let preference = CustomMenuItem::new("preference".to_string(), "设置");
    let refresh = CustomMenuItem::new("refresh", "刷新");
    // let always_top = CustomMenuItem::new("always_top", "常驻置顶").selected();
    SystemTrayMenu::new()
        .add_item(open)
        .add_item(refresh)
        // .add_item(always_top)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(ernie_bot)
        .add_item(chat_chat)
        .add_item(chat_gpt)
        .add_item(chat_gpt_free2)
        .add_item(chat_gpt_free3)
        .add_item(chat_gpt_official)
        .add_item(bing)
        .add_item(poe)
        .add_item(bard)
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
    let chat_chat = CustomMenuItem::new("chat_chat".to_string(), "ChatGPT(限额版)");
    let poe = CustomMenuItem::new("poe".to_string(), "POE");
    let bing = CustomMenuItem::new("bing".to_string(), "NewBing");
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
            .add_item(chat_gpt_free2)
            .add_item(chat_gpt_free3)
            .add_item(chat_gpt_official)
            .add_item(bing)
            .add_item(poe)
            .add_item(bard),
    );
    let about_submenu = Submenu::new(
        "更多".to_string(),
        Menu::new().add_item(github).add_item(gitee),
    );

    Menu::new()
        .add_submenu(mode_submenu)
        .add_item(CustomMenuItem::new("refresh", "刷新"))
        .add_item(CustomMenuItem::new("preference", "设置"))
        .add_submenu(about_submenu)
}

/// 窗口事件
pub fn on_window_event_handler(event: WindowMenuEvent) {
    // 窗口菜单监听
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
        "chat_gpt_free2" => {
            event
                .window()
                .eval("window.location.replace('https://chatbot.theb.ai/')")
                .unwrap();
        }
        "chat_gpt_free3" => {
            event
                .window()
                .eval("window.location.replace('https://chatgpt-35-turbo.com/')")
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
}
