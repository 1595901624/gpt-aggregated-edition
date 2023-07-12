
#![allow(dead_code)]
#![allow(unused_variables)]

use tauri::App;

pub fn on_global_event(app: &App) {
    // 更新窗口菜单
    // let id = app.listen_global("update-window-menu", |event| {
    //     let main_window = app.get_window("main").unwrap();
    //     let menu_handle = main_window.menu_handle();
    //     std::thread::spawn(move || {
    //         menu_handle.get_item("item_id").set_title("New title");
    //     });
    //     // println!("got event-name with payload {:?}", event.payload());
    // });
}
