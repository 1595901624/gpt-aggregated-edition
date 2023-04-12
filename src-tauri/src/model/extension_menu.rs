/// 用户扩展的菜单
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ExtensionMenu {
    id: Option<i32>,
    name: Option<String>,
    url: Option<String>,
}

impl ExtensionMenu {
    /// 获取id
    // pub fn get_hash_id(&self) -> String {
    //     let mut handler = DefaultHasher::new();
    //     self.name.hash(&mut handler);
    //     self.url.hash(&mut handler);
    //     return format!("hy_{}", handler.finish());
    // }

    pub fn get_id(&self) -> Option<i32> {
        self.id
    }

    pub fn get_string_id(&self) -> Option<String> {
        if let Some(id) = self.get_id() {
            return Some(format!("hy_cus_{}", id));
        }
        return None;
    }

    pub fn get_name(&self) -> Option<String> {
        return self.name.clone();
    }

    pub fn get_url(&self) -> Option<String> {
        return self.url.clone();
    }
}
