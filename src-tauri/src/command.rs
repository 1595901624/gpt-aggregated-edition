#[allow(dead_code)]
use docx_rust::{
    document::{BookmarkEnd, BookmarkStart, BreakType, Paragraph, Run, TextSpace},
    formatting::{CharacterProperty, JustificationVal, ParagraphProperty},
    styles::{Style, StyleType},
    Docx,
};

use crate::{model::chat_content::ChatContent, preference_util};

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

/// 获取
#[tauri::command]
pub fn get_window_mode_handler() -> i32 {
    preference_util::get_window_mode() as i32
}

/// 设置模式
#[tauri::command]
pub fn set_window_mode_handler(mode: i32) {
    preference_util::set_window_mode(mode);
}

#[tauri::command]
pub fn is_enable_internal_script_handler() -> bool {
    preference_util::is_enable_internal_script()
}

#[tauri::command]
pub fn enable_internal_script_handler(enable: bool) {
    preference_util::enable_internal_script(enable);
}

/// 设置设置项
#[tauri::command]
pub fn set_preference_handler(key: i32, value: &str) -> bool {
    return preference_util::set_preference(key, value);
}

/// 获取设置项
#[tauri::command]
pub fn get_preference_handler(key: i32, value: &str) -> String {
    return preference_util::get_preference(key, value);
}

/// 创建docx文档
#[tauri::command]
pub fn create_docx_handler(value: &str) -> String {
    let mut docx = Docx::default();
    // let font = Font::new("Arial")
    //     .charset("00")
    //     .family("swiss")
    //     .pitch("variable");
    dbg!(value);

    // 创建标题样式
    docx.styles.push(
        Style::new(StyleType::Paragraph, "HeaderStyle")
            .name("Header Style")
            .character(
                CharacterProperty::default()
                    .bold(true)
                    .size(42isize)
                    .color(0x000000),
            ),
    );

    // docx.styles.default(
    //     DefaultStyle::default().character(
    //         CharacterProperty::default()
    //             .size(42isize)
    //             .color((0x00, 0xff, 0x00)),
    //     ),
    // );

    let title = Paragraph::default()
        .property(
            ParagraphProperty::default()
                .style_id("HeaderStyle")
                .justification(JustificationVal::Center),
        )
        .push(
            Run::default()
                .property(CharacterProperty::default().style_id("HeaderStyle"))
                .push_text("文心一言对话"),
        );

    docx.document.push(title);

    let subtitle = Paragraph::default()
        .property(ParagraphProperty::default().justification(JustificationVal::Center))
        .push(
            Run::default()
                .property(CharacterProperty::default().size(18isize).color(0x2f2f2f))
                .push_text("本文档由 OneGpt 自动生成，如有非法等不良内容，与 OneGPT 无关。")
                .push_break(BreakType::TextWrapping),
        );

    docx.document.push(subtitle);

    let para = Paragraph::default()
        .property(ParagraphProperty::default())
        .push_text("Q:")
        .push_text((" 孙悟空是碳基生物还是硅基生物？", TextSpace::Default))
        .push(Run::default().push_text("content"))
        .push(BookmarkStart::default())
        .push(BookmarkEnd::default());

    // let style = Style::new(StyleType::Paragraph, "style_id")
    //     .name("Style Name")
    //     .paragraph(ParagraphProperty::default())
    //     .character(CharacterProperty::default());

    // let para = Paragraph::default()
    //     .push(paragraph_content)
    //     .push_text("孙悟空是碳基生物还是硅基生物？");
    docx.document.push(para);
    let para = Paragraph::default().push_text(r#"孙悟空是碳基生物。

    在原著《西游记》中，孙悟空曾经被训练过的妖精说过：“大圣爷爷，您是个石猴，是石头里蹦出来的，石头是碳基生命，所以您肯定是碳基生物。” 同时，在原著的《大闹天宫》一章中也有提到，孙悟空的真身是猴子，石头的成分也是碳，由此可见，孙悟空应该是以碳基生命形式存在的。"#);
    docx.document.push(para);

    let path = preference_util::get_app_config_root_path().join("demo.docx");
    println!("{}", path.to_str().unwrap());
    docx.write_file(path).unwrap();
    return "".to_string();
}

/// 生成markdown
#[tauri::command]
pub fn create_markdown_handler(title: &str, content_json: &str) -> String {
    let mut content = String::new();

    // 内容
    if let Ok(markdown_content_list) = serde_json::from_str::<Vec<ChatContent>>(content_json) {
        for i in 0..markdown_content_list.len() {
            let item = &markdown_content_list[i];
            (&mut content).push_str("#### 对话 ");
            (&mut content).push_str((i + 1).to_string().as_str());
            (&mut content).push_str(":\n##### 提问: ");
            (&mut content).push_str(&item.question);
            (&mut content).push_str("\n");

            (&mut content).push_str("##### 回答: ");
            if let Some(img) = &item.answer_image {
                // ![](http://aaa);
                (&mut content).push_str("![](");
                (&mut content).push_str(img);
                (&mut content).push_str(")");
                (&mut content).push_str("\n\n");
            }
            (&mut content).push_str(&item.answer);
            (&mut content).push_str("\n");
        }

        // markdown_content_list.iter().for_each(|item| {
        //     (&mut content).push_str("#### Question:");
        //     (&mut content).push_str(&item.question);
        //     (&mut content).push_str("\n\n");
        //     if let Some(img) = &item.answer_image {
        //         // ![](http://aaa);
        //         (&mut content).push_str("![](");
        //         (&mut content).push_str(img);
        //         (&mut content).push_str(")");
        //         (&mut content).push_str("\n\n");
        //     }
        //     (&mut content).push_str(&item.answer);
        //     (&mut content).push_str("\n");
        // });
    }
    format!("{}{}", title, content)
}
