// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod command;
mod menu;
mod model;
mod plugin;
mod preference_util;
mod event;
mod util;

// use docx_rust::{
//     document::{
//         BookmarkEnd, BookmarkStart, BreakType, Paragraph, ParagraphContent, Run, TextSpace,
//     },
//     font_table::Font,
//     formatting::{CharacterProperty, JustificationVal, ParagraphProperty},
//     styles::{Style, StyleType},
//     Docx,
// };
use log::info;
use model::{constant, preference_model::WindowMode};
use tauri::{generate_handler, GlobalShortcutManager, Manager, SystemTray, LogicalSize};
use tauri_plugin_log::LogTarget;
use tauri_plugin_positioner::{Position, WindowExt};

fn main() {
    // 初始化设置项
    preference_util::init_default_preference();

    // 初始化右下角菜单
    let tray = SystemTray::new().with_menu(menu::create_tary_menu());

    let context = tauri::generate_context!();

    // test2();
    
    // 初始化窗口
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([LogTarget::LogDir, LogTarget::Stdout, LogTarget::Webview])
                .build(),
        )
        .invoke_handler(generate_handler![
            command::greet,
            command::get_window_mode_handler,
            command::set_window_mode_handler,
            command::is_enable_internal_script_handler,
            command::enable_internal_script_handler,
            command::create_markdown_handler,
            command::set_preference_handler,
            command::get_preference_handler,
            command::get_app_preference_handler,
            command::create_docx_handler,
            command::create_markdown_handler,
            command::query_extension_menus_handler,
            command::add_extension_menu_item_handler,
            command::edit_extension_menu_item_handler,
            command::delete_extension_menu_item_handler,
        ])
        .plugin(tauri_plugin_positioner::init())
        .setup(|app| {
            info!("[tauri setup]");
            event::on_global_event(&app);

            let url = preference_util::get_preference(constant::PREFERENCE_CURRENT_PAGE_URL, "");
            let main_window_builder =
                tauri::WindowBuilder::new(app, "main", tauri::WindowUrl::App(url.into()))
                    .title(constant::APP_NAME)
                    .enable_clipboard_access()
                    .visible(false);
            let main_window;
            if preference_util::is_enable_internal_script() {
                main_window = main_window_builder
                    .initialization_script(include_str!("../plugin/base.js"))
                    // .initialization_script(include_str!("./plugin/third/html2canvas.js"))
                    .initialization_script(include_str!("../plugin/erniebot.js"))
                    .initialization_script(include_str!("../plugin/chatchat.js"))
                    .menu(menu::create_window_menu())
                    .build()
                    .unwrap();
            } else {
                main_window = main_window_builder.build().unwrap();
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
                let main_window = app.get_window("main").unwrap();
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
        })
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
        .on_menu_event(menu::on_window_event_handler)
        // 任务栏菜单监听
        .on_system_tray_event(menu::on_tray_event)
        .run(context)
        .expect("error while running tauri application");
}

// fn test() {
//     let mut docx = Docx::default();
//     // let font = Font::new("Arial")
//     //     .charset("00")
//     //     .family("swiss")
//     //     .pitch("variable");

//     // 创建标题样式
//     docx.styles.push(
//         Style::new(StyleType::Paragraph, "HeaderStyle")
//             .name("Header Style")
//             .character(
//                 CharacterProperty::default()
//                     .bold(true)
//                     .size(42isize)
//                     .color(0x000000),
//             ),
//     );

//     // docx.styles.default(
//     //     DefaultStyle::default().character(
//     //         CharacterProperty::default()
//     //             .size(42isize)
//     //             .color((0x00, 0xff, 0x00)),
//     //     ),
//     // );

//     let title = Paragraph::default()
//         .property(
//             ParagraphProperty::default()
//                 .style_id("HeaderStyle")
//                 .justification(JustificationVal::Center),
//         )
//         .push(
//             Run::default()
//                 .property(CharacterProperty::default().style_id("HeaderStyle"))
//                 .push_text("文心一言对话"),
//         );

//     docx.document.push(title);

//     let subtitle = Paragraph::default()
//         .property(ParagraphProperty::default().justification(JustificationVal::Center))
//         .push(
//             Run::default()
//                 .property(CharacterProperty::default().size(18isize).color(0x2f2f2f))
//                 .push_text("本文档由 OneGpt 自动生成，如有非法等不良内容，与 OneGPT 无关。")
//                 .push_break(BreakType::TextWrapping),
//         );

//     docx.document.push(subtitle);

//     let para = Paragraph::default()
//         .property(ParagraphProperty::default())
//         .push_text("Q:")
//         .push_text((" 孙悟空是碳基生物还是硅基生物？", TextSpace::Default))
//         .push(Run::default().push_text("content"))
//         .push(BookmarkStart::default())
//         .push(BookmarkEnd::default());

//     // let style = Style::new(StyleType::Paragraph, "style_id")
//     //     .name("Style Name")
//     //     .paragraph(ParagraphProperty::default())
//     //     .character(CharacterProperty::default());

//     // let para = Paragraph::default()
//     //     .push(paragraph_content)
//     //     .push_text("孙悟空是碳基生物还是硅基生物？");
//     docx.document.push(para);
//     let para = Paragraph::default().push_text(r#"孙悟空是碳基生物。

//     在原著《西游记》中，孙悟空曾经被训练过的妖精说过：“大圣爷爷，您是个石猴，是石头里蹦出来的，石头是碳基生命，所以您肯定是碳基生物。” 同时，在原著的《大闹天宫》一章中也有提到，孙悟空的真身是猴子，石头的成分也是碳，由此可见，孙悟空应该是以碳基生命形式存在的。"#);
//     docx.document.push(para);

//     let path = preference_util::get_app_config_root_path().join("demo.docx");
//     println!("{}", path.to_str().unwrap());
//     docx.write_file(path).unwrap();
// }

// fn test2() {
//     let content_json = r#"[{"answerImage":"http://eb118-file.cdn.bcebos.com/upload/6A594000B509CBDB6DF2EBA7DF87A53C?","question":"画一个鸭梨#创意图#","answer":"好的，根据你的需求，我为你创作了一幅画作。\n我的作画技能还在不断进化中，暂时还不支持对画作的修改和解释。\n如果需要继续让我为你作画，请完整描述你的需求，如：“帮我画一枝晶莹剔透的牡丹花”。"}]"#;
//     let title = format!("### {}\n\n", "测试");
//     let mut content = String::new();

//     // 内容
//     if let Ok(markdown_content_list) = serde_json::from_str::<Vec<ChatContent>>(content_json) {
//         for i in 0..markdown_content_list.len() {
//             let item = &markdown_content_list[i];
//             (&mut content).push_str("#### 对话 ");
//             (&mut content).push_str((i + 1).to_string().as_str());
//             (&mut content).push_str(":\n##### 提问: ");
//             (&mut content).push_str(&item.question);
//             (&mut content).push_str("\n");

//             (&mut content).push_str("##### 回答: ");
//             if let Some(img) = &item.answer_image {
//                 // ![](http://aaa);
//                 (&mut content).push_str("![](");
//                 (&mut content).push_str(img);
//                 (&mut content).push_str(")");
//                 (&mut content).push_str("\n\n");
//             }
//             (&mut content).push_str(&item.answer);
//             (&mut content).push_str("\n");
//         }

//         // markdown_content_list.iter().for_each(|item| {
//         //     (&mut content).push_str("#### Question:");
//         //     (&mut content).push_str(&item.question);
//         //     (&mut content).push_str("\n\n");
//         //     if let Some(img) = &item.answer_image {
//         //         // ![](http://aaa);
//         //         (&mut content).push_str("![](");
//         //         (&mut content).push_str(img);
//         //         (&mut content).push_str(")");
//         //         (&mut content).push_str("\n\n");
//         //     }
//         //     (&mut content).push_str(&item.answer);
//         //     (&mut content).push_str("\n");
//         // });
//     }
//     println!("{}{}", title, content);
//     // return title + content.as_str();
// }
