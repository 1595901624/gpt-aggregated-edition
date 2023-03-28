use crate::preference_util;

/// 读取内置脚本插件
pub fn load_internal_plugin() -> String {
    if !preference_util::is_enable_internal_script() {
        return "".into();
    }
    let js = read("./plugin/unecrypt.js");
    return js;
}

pub fn load_system_js() -> String {
    let key = r#"
    window.oncontextmenu=function(e){{
        e.preventDefault();
    }}
    "#;
    return key.into();
}

/// 读取用户自定义脚本
// pub fn load_custom_plugin() -> String {
//     return read("").into();
// }

/// 读取路径下的代码
fn read(path: &str) -> String {
    let js = std::fs::read(path);
    if js.is_err() {
        dbg!("error");
        return "".into();
    }
    return String::from_utf8(js.unwrap()).unwrap();
}
