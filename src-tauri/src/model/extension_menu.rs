use std::{collections::hash_map::DefaultHasher, hash::{Hash, Hasher}};


/// 用户扩展的菜单
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ExtensionMenu<'a> {
    menu_name: Option<&'a str>,
    menu_url: Option<&'a str>,
}

impl<'a> ExtensionMenu<'a> {

    /// 获取id
    pub fn get_id(&self) -> String {
        let mut handler = DefaultHasher::new();
        self.menu_name.hash(&mut handler);
        self.menu_url.hash(&mut handler);
        return format!("hy{}", handler.finish());
    }

    pub fn get_menu_name(&self) -> Option<&str>{
        return self.menu_name;
    }

    pub fn get_menu_url(&self) -> Option<&str> {
        return self.menu_url;
    }
}
