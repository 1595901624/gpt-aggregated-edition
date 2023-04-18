#![allow(dead_code)]
#![allow(unused_variables)]

/// 用户扩展的菜单
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ExtensionMenu {
    id: Option<i32>,
    name: Option<String>,
    url: Option<String>,
    // 优先级
    priority: Option<i32>,
    separator: Option<bool>,
    submenu: Option<Vec<ExtensionMenu>>,
}

impl ExtensionMenu {
    /// 获取id
    // pub fn get_hash_id(&self) -> String {
    //     let mut handler = DefaultHasher::new();
    //     self.name.hash(&mut handler);
    //     self.url.hash(&mut handler);
    //     return format!("hy_{}", handler.finish());
    // }

    pub fn new(id: i32, name: String, url: String, priority: i32) -> Self {
        Self {
            id: Some(id),
            name: Some(name),
            url: Some(url),
            priority: Some(priority),
            ..Default::default()
        }
    }

    pub fn get_id(&self) -> i32 {
        self.id.unwrap_or_default()
    }

    pub fn get_string_id(&self) -> Option<String> {
        return Some(format!("hy_cus_{}", self.get_id()));
    }

    pub fn set_id(&mut self, id: i32) {
        self.id = Some(id);
    }

    pub fn get_name(&self) -> Option<String> {
        return self.name.clone();
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn get_url(&self) -> Option<String> {
        return self.url.clone();
    }

    pub fn set_url(&mut self, url: String) {
        self.url = Some(url);
    }

    pub fn get_priority(&self) -> Option<i32> {
        self.priority
    }

    pub fn set_priority(&mut self, priority: i32) {
        self.priority = Some(priority);
    }

    pub fn is_separator(&self) -> bool {
        self.separator.unwrap_or_default()
    }

    pub fn exist_submenu(&self) -> bool {
        if let Some(_) = self.submenu {
            return true;
        }
        return false;
    }

    pub fn get_submenu(&self) -> Vec<ExtensionMenu> {
        self.submenu.clone().unwrap_or_else(|| vec![])
    }

    // 是否需要添加监听器
    pub fn is_add_listener(&self) -> bool {
        self.id.is_some() && self.url.is_some()
    }
}

/// 父级菜单
/// 显示在窗口或者右下角的一级菜单
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ParentMenu {
    id: Option<i32>,
    title: Option<String>,
    // 优先级
    priority: Option<i32>,
    separator: Option<bool>,
    menu: Option<Vec<ExtensionMenu>>,
}

impl ParentMenu {
    pub fn get_id(&self) -> i32 {
        self.id.unwrap_or_default()
    }

    pub fn get_string_id(&self) -> String {
        return format!("main_{}", self.get_id());
    }

    pub fn get_title(&self) -> String {
        return self.title.clone().unwrap_or_else(|| "未设置值".to_string());
    }

    // fn new(id: i32, title: String) -> Self {}
    pub fn is_separator(&self) -> bool {
        self.separator.unwrap_or_default()
    }

    pub fn get_priority(&self) -> i32 {
        self.priority.unwrap_or_default()
    }

    pub fn exist_menu(&self) -> bool {
        if let Some(_) = self.menu {
            return true;
        }
        return false;
    }

    pub fn get_menu(&self) -> Vec<ExtensionMenu> {
        self.menu.clone().unwrap_or_else(|| vec![])
    }
}
