#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use tauri::{
    api, CustomMenuItem, GlobalShortcutManager, Manager, SystemTray, SystemTrayEvent,
    SystemTrayMenu, SystemTrayMenuItem,
};
use tauri_plugin_positioner::{Position, WindowExt};

fn main() {
    let inject_script = r#"
    if (window.location.href.includes("yiyan.baidu.com")) {
    // 文心一言
    const style = document.createElement('style');
    style.innerHTML = `.ebhelper-hide { visibility: hidden !important; }`;
    document.head.appendChild(style);

    // ai图片水印标记
    const aiImageWaterFlag = "x-bce-process=style/wm_ai";

    // 创建一个MutationObserver实例
    const observer = new MutationObserver(function (mutations) {
        // 获取水印元素
        let watermark = document.querySelector("div[id^='eb_']");
        if (watermark != null && !watermark.classList.contains('ebhelper-hide')) {
            hideWatermark(watermark);
        }

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
    });

    // 开始观察document，并在节点添加或删除时检测变化
    observer.observe(document, {
        childList: true,
        subtree: true
    });


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
        console.log("隐藏水印!");
        element.classList.add('ebhelper-hide');
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
}
    "#;

    let open = CustomMenuItem::new("open".to_string(), "打开窗口").accelerator("Cmd+Shift+O");
    let quit = CustomMenuItem::new("quit".to_string(), "退出").accelerator("Cmd+Q");
    let chat_gpt = CustomMenuItem::new("chat_gpt".to_string(), "ChatGPT(免费版)");
    let chat_gpt_official = CustomMenuItem::new("chat_gpt_official".to_string(), "ChatGPT(官方版)");
    let ernie_bot = CustomMenuItem::new("ernie_bot".to_string(), "文心一言");
    let github = CustomMenuItem::new("github".to_string(), "访问 Github");
    let gitee = CustomMenuItem::new("gitee".to_string(), "访问 Gitee");
    let tray_menu = SystemTrayMenu::new()
        .add_item(open)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(ernie_bot)
        .add_item(chat_gpt)
        .add_item(chat_gpt_official)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(github)
        .add_item(gitee)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    let tray = SystemTray::new().with_menu(tray_menu);

    let context = tauri::generate_context!();

    tauri::Builder::default()
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

            let main_window = app.get_window("main").unwrap();
            main_window.show().unwrap();
            main_window.set_focus().unwrap();
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
                if !is_focused {
                    event.window().hide().unwrap();
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
                    let _ = window.move_window(Position::TrayCenter);

                    if window.is_visible().unwrap() {
                        window.hide().unwrap();
                    } else {
                        window.show().unwrap();
                        window.set_focus().unwrap();

                        window
                            .eval(inject_script)
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
                        main_window.eval(&format!(
                            "window.location.replace('https://yiyan.baidu.com/')"
                        ));
                    }
                    "chat_gpt" => {
                        let main_window = app.get_window("main").unwrap();
                        main_window.show().unwrap();
                        main_window.set_focus().unwrap();
                        main_window.eval(&format!(
                            "window.location.replace('https://freegpt.one/')"
                        ));
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
                        main_window.eval(&format!(
                            "window.location.replace('https://chat.openai.com/chat')"
                        ));
                    }
                    "quit" => {
                        std::process::exit(0);
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
