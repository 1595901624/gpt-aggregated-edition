/// 读取文心一言插件
pub fn load_eb_plugin() -> String {
    let js = read("./plugin/internal.js");
    // dbg!(&result);
    return js;
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
