// use crate::preference_util;

// /// 读取内置脚本
// pub fn load_internal_script(path: &str) -> String {
//     if !preference_util::is_enable_internal_script() {
//         return "".into();
//     }
//     let js = read(path);
//     return js;
// }

// /// 读取用户自定义脚本
// pub fn load_custom_plugin() -> String {
//     return read("").into();
// }

// // 读取路径下的代码
// fn read(path: &str) -> String {
//     let js = std::fs::read(path);
//     if js.is_err() {
//         dbg!("error");
//         return "".into();
//     }
//     return String::from_utf8(js.unwrap()).unwrap();
// }
