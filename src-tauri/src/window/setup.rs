use log::info;
use tauri::{App, LogicalSize, GlobalShortcutManager, Manager};
use tauri_plugin_positioner::{Position, WindowExt};

use crate::{
    model::{constant::{self, WINDOW_LABEL_MAIN}, preference_model::WindowMode},
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
            .title(constant::APP_NAME)
            .enable_clipboard_access()
            .visible(false);
    let main_window;
    if preference_util::is_enable_internal_script() {
        main_window = main_window_builder
            .initialization_script(include_str!("../../plugin/base.js"))
            // .initialization_script(include_str!("./plugin/third/html2canvas.js"))
            .initialization_script(include_str!("../../plugin/erniebot.js"))
            .initialization_script(include_str!("../../plugin/chatchat.js"))
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
    }
    Ok(())
}
