// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod model;
mod preference_util;

use model::preference_model::WindowMode;
use tauri::{
    api, generate_handler, CustomMenuItem, GlobalShortcutManager, LogicalSize, Manager, SystemTray,
    SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};
use tauri_plugin_positioner::{Position, WindowExt};

fn main() {
    preference_util::init_default_preference();

    let inject_yiyan_script = r#"
    if (window.location.href.includes("yiyan.baidu.com")) {
        // 文心一言
    
        // ai图片水印标记
        const aiImageWaterFlag = "x-bce-process=style/wm_ai";
     
        // 打开shadow-root
        Element.prototype._attachShadow = Element.prototype.attachShadow;
        Element.prototype.attachShadow = function () {
            return this._attachShadow({ mode: "open" });
        };
     
        // 自己调整频次 感觉10毫秒比较友好
        setInterval(deal, 10);
     
        function deal() {
            // 获取水印元素
            let watermark = getElementByRegex(/^[\w\d]{8}-[\w\d]{4}-[\w\d]{4}-[\w\d]{4}-[\w\d]{12}$/);
            if (watermark != null && watermark.classList != null && !watermark.classList.contains('ebhelper-hide')) {
                hideWatermark(watermark);
            }
            // Array.from(document.querySelectorAll('div')).filter(e => e.shadowRoot);
     
            // 获取弹窗的元素
            let timeoutDialog = document.querySelector("div[class='ant-modal-root']");
            if (timeoutDialog != null && !timeoutDialog.classList.contains('ebhelper-hide')) {
                hideTimeoutDialog(timeoutDialog);
            }
     
            // 隐藏图片水印并处理头像
            let allImage = document.querySelectorAll("img");
            if (allImage != null) {
                hideAIImageWatermark(allImage);
            }
        }
     
     
        /**
         * 隐藏超时弹窗
         */
        function hideTimeoutDialog(element) {
            console.log("隐藏超时弹窗!");
            element.classList.add('ebhelper-hide');
        }
     
     
        /**
         * 隐藏水印
         */
        function hideWatermark(element) {
            // console.log("隐藏水印!");
            let shadows = element.shadowRoot.querySelectorAll('*');
            for (let e of shadows) {
                e.remove();
            }
        }
     
        /**
         * 隐藏图片水印并处理头像
         */
        function hideAIImageWatermark(images) {
            images.forEach(element => {
                let url = element.getAttribute("src");
                // 去除水印
                if (url != null && url.indexOf(aiImageWaterFlag) != -1) {
                    if (url.indexOf(aiImageWaterFlag) != -1) {
                        console.log("隐藏图片水印!");
                        element.setAttribute("src", url.replace(aiImageWaterFlag, ""))
                    }
                }
                // 处理头像
                if (url != null
                    && element.getAttribute("alt") == '头像'
                    && url.indexOf('icon-rb') == '-1') {
                    console.log("设置头像为默认值!");
                    element.setAttribute("src", 'https://nlp-eb.cdn.bcebos.com/logo/favicon.ico')
                }
            });
        }
     
        /**
         * 正则匹配元素,获取第一个元素
         * @param {*} pattern 
         * @returns 
         */
        function getElementByRegex(pattern) {
            let allElements = document.getElementsByTagName('div');
            let result = "";
     
            for (let i = 0; i < allElements.length; i++) {
                let element = allElements[i];
                let attr = element.getAttribute('id');
                if (attr != null && pattern.test(attr)) {
                    result = element;
                    break;
                }
            }
     
            return result;
        }
    }
    
        "#;

    let open = CustomMenuItem::new("open".to_string(), "打开窗口").accelerator("Cmd+Shift+O");
    let quit = CustomMenuItem::new("quit".to_string(), "退出").accelerator("Cmd+Q");
    let chat_gpt = CustomMenuItem::new("chat_gpt".to_string(), "ChatGPT(免费版)");
    let chat_gpt_official = CustomMenuItem::new("chat_gpt_official".to_string(), "ChatGPT(官方版)");
    let ernie_bot = CustomMenuItem::new("ernie_bot".to_string(), "文心一言");
    let github = CustomMenuItem::new("github".to_string(), "访问 Github");
    let gitee = CustomMenuItem::new("gitee".to_string(), "访问 Gitee");
    let preference = CustomMenuItem::new("preference".to_string(), "设置");
    let tray_menu = SystemTrayMenu::new()
        .add_item(open)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(ernie_bot)
        .add_item(chat_gpt)
        .add_item(chat_gpt_official)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(github)
        .add_item(gitee)
        .add_item(preference)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    let tray = SystemTray::new().with_menu(tray_menu);

    let context = tauri::generate_context!();

    // 初始化窗口
    tauri::Builder::default()
        .invoke_handler(generate_handler![
            greet,
            get_window_mode_handler,
            set_window_mode_handler
        ])
        .plugin(tauri_plugin_positioner::init())
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();

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
                main_window.eval(inject_yiyan_script).unwrap();
                main_window.move_window(Position::Center).unwrap();
                main_window.set_size(LogicalSize::new(800, 600)).unwrap();
                main_window.set_decorations(true).unwrap();
                main_window.set_always_on_top(false).unwrap();
                main_window.set_skip_taskbar(false).unwrap();
                main_window.menu_handle().hide().unwrap();
                main_window.show().unwrap();
                main_window.set_focus().unwrap();
            }
            Ok(())
        })
        .menu(tauri::Menu::os_default(&context.package_info().name))
        .system_tray(tray)
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
        .on_system_tray_event(|app, event| {
            tauri_plugin_positioner::on_tray_event(app, &event);
            match event {
                SystemTrayEvent::LeftClick {
                    position: _,
                    size: _,
                    ..
                } => {
                    let window = app.get_window("main").unwrap();

                    let mode = preference_util::get_window_mode();
                    if mode == WindowMode::TaskBar {
                        // 任务栏模式
                        window.move_window(Position::TrayCenter).unwrap();
                        window.set_size(LogicalSize::new(450, 600)).unwrap();
                        window.set_decorations(false).unwrap();
                        window.set_always_on_top(true).unwrap();
                        window.set_skip_taskbar(true).unwrap();
                        window.menu_handle().hide().unwrap();
                    } else {
                        // 桌面模式
                        window.move_window(Position::Center).unwrap();
                        window.set_size(LogicalSize::new(800, 600)).unwrap();
                        window.set_decorations(true).unwrap();
                        window.set_always_on_top(false).unwrap();
                        window.set_skip_taskbar(false).unwrap();
                        window.menu_handle().hide().unwrap();
                    }

                    if window.is_visible().unwrap() {
                        window.hide().unwrap();
                    } else {
                        window.show().unwrap();
                        window.set_focus().unwrap();

                        window
                            .eval(inject_yiyan_script)
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
                        //main_window.eval("window.location.reload");
                    }
                    "chat_gpt" => {
                        let main_window = app.get_window("main").unwrap();
                        main_window.show().unwrap();
                        main_window.set_focus().unwrap();
                        // main_window
                        //     .eval(&format!("window.location.replace('https://freegpt.one/')"));
                        main_window
                            .eval("window.location.href = 'https://freegpt.one/'")
                            .unwrap();
                        // let main_window = app.get_window("main").unwrap();
                        // main_window.show().unwrap();
                        // main_window.set_focus().unwrap();
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
                    "quit" => {
                        std::process::exit(0);
                    }
                    "preference" => {
                        let preference_window = app.get_window("preference").unwrap();
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

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

/// 获取
#[tauri::command]
fn get_window_mode_handler() -> i32 {
    preference_util::get_window_mode() as i32
}

#[tauri::command]
fn set_window_mode_handler(mode: i32) {
    preference_util::set_window_mode(mode);
}
