use log::info;
use tauri::{App, GlobalShortcutManager, LogicalSize, Manager, PhysicalPosition, PhysicalSize};
use tauri_plugin_positioner::{Position, WindowExt};

use crate::{
    model::{
        constant::{self, WINDOW_LABEL_MAIN},
        preference_model::WindowMode,
    },
    preference_util, window,
};

pub fn setup(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    info!("[tauri setup]");
    window::event::on_global_event(&app);

    let url = preference_util::get_preference(
        constant::PREFERENCE_CURRENT_PAGE_URL,
        "https://yiyan.baidu.com",
    );
    info!("[init url] => {}", &url);
    let main_window_builder =
        tauri::WindowBuilder::new(app, WINDOW_LABEL_MAIN, tauri::WindowUrl::App(url.into()))
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Safari/537.36 Edg/112.0.1722.58")
            .title(constant::APP_NAME)
            .enable_clipboard_access()
            .visible(false);
    let main_window;
    if preference_util::is_enable_internal_script() {
        main_window = main_window_builder
            // .user_agent(user_agent)
            .initialization_script(include_str!("../../plugin/base.js"))
            // .initialization_script(include_str!("./plugin/third/html2canvas.js"))
            .initialization_script(include_str!("../../plugin/gptai.js"))
            .initialization_script(include_str!("../../plugin/erniebot.js"))
            .initialization_script(include_str!("../../plugin/chatchat.js"))
            .initialization_script(include_str!("../../plugin/chatbot.js"))
            .initialization_script(include_str!("../../plugin/newbing.js"))
            .menu(window::menu::create_window_menu())
            .build()
            .unwrap();
    } else {
        main_window = main_window_builder
            .menu(window::menu::create_window_menu())
            .build()
            .unwrap();
    }

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
        let main_window = app.get_window(WINDOW_LABEL_MAIN).unwrap();
        let init_window_mutex = constant::INIT_WINDOW_MODE.try_lock().unwrap();
        main_window
            .set_size(LogicalSize::new(
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
        init_window_mutex.set(true);
    } else if preference_util::get_window_mode() == WindowMode::QQ {
        let main_window = app.get_window(WINDOW_LABEL_MAIN).unwrap();
        let init_qq_mutex = constant::INIT_QQ_MODE.try_lock().unwrap();
        let screen = main_window.current_monitor().unwrap().unwrap();
        let screen_height = screen.size().height as i32;
        let screen_width = screen.size().width as i32;

        let physical_width = constant::SIDE_BAR_WIDTH as f64 * main_window.scale_factor().unwrap();
        main_window
            .set_size(PhysicalSize::new(
                physical_width as i32,
                screen_height - 500,
            ))
            .unwrap();
        main_window
            .set_position(PhysicalPosition::new(
                screen_width - main_window.outer_size().unwrap().width as i32 - 100,
                (screen_height - main_window.outer_size().unwrap().height as i32) / 2,
            ))
            .unwrap();

        main_window.set_decorations(true).unwrap();
        main_window.set_always_on_top(true).unwrap();
        main_window.set_skip_taskbar(true).unwrap();
        main_window.menu_handle().show().unwrap();
        main_window.show().unwrap();
        main_window.set_focus().unwrap();
        init_qq_mutex.set(true);
    }
    Ok(())
}
