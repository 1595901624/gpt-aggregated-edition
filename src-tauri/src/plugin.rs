/// 读取文心一言插件
pub fn load_eb_plugin() -> String {
    let js = std::fs::read("./plugin/eb.js");
    if js.is_err() {
        dbg!("error");
        return "".to_string();
    }
    let result = format!(
        r#"if (window.location.href.includes("yiyan.baidu.com")) {{
            {}
        }}"#,
        String::from_utf8(js.unwrap()).unwrap()
    );
    dbg!(&result);
    return result;
}
