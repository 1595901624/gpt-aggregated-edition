//// 切换页面后再次执行脚本的时间
// pub const SWITCH_PAGE_SLEEP_TIME: u64 = 500;

use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
};

use tauri::async_runtime::Mutex;

// 窗口模式的宽
pub const WINDOW_WIDTH: i32 = 1000;
// 窗口模式的高
pub const WINDOW_HEIGHT: i32 = 750;

// 任务栏模式的宽
pub const TASK_WINDOW_WIDTH: i32 = 400;
// 任务栏模式的高
pub const TASK_WINDOW_HEIGHT: i32 = 750;

// 侧边栏宽度
pub const SIDE_BAR_WIDTH: i32 = 400;

// App名称
pub const APP_NAME: &str = "OneGPT";

// ***************************Window START***************************** //
// 主窗口
pub const WINDOW_LABEL_MAIN: &str = "main";
// 设置窗口
pub const WINDOW_LABEL_PREFERENCE: &str = "preference";
// ***************************Window E N D***************************** //

// ***************************设置项 START***************************** //
pub const PREFERENCE_AUTO_HIDE_WHEN_CLICK_OUTSIDE: i32 = 3;
pub const PREFERENCE_CURRENT_PAGE_URL: i32 = 4;
pub const PREFERENCE_EXIT_APP: i32 = 5;
// ***************************设置项 E N D***************************** //

// 当前是否是最小化
// pub const BRFORE_WINDOW_MINIMIZED_SIZE: Cell<(u32, u32)> = Cell::new((0, 0));
// pub const BRFORE_WINDOW_MINIMIZED_POSITION: Cell<(i32, i32)> = Cell::new((0, 0));

// pub const MENU_MAP: HashMap<String, ExtensionMenu> = HashMap::new();
lazy_static! {
    // 菜单缓存
    pub static ref MENU_MAP: Mutex<RefCell<HashMap<String, String>>> = Mutex::new(RefCell::new(HashMap::new()));

    // 当前状态是否是最小化
    pub static ref IS_MINIMIZED: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));

    // 是否已经初始过QQ模式
    pub static ref INIT_QQ_MODE: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));

    // 是否已经初始过窗口模式
    pub static ref INIT_WINDOW_MODE: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));

    // pub static ref BRFORE_WINDOW_MINIMIZED_SIZE: Mutex<Cell<(u32, u32)>> = Mutex::new(Cell::new((0, 0)));
    // pub static ref BRFORE_WINDOW_MINIMIZED_POSITION: Mutex<Cell<(i32, i32)>> = Mutex::new(Cell::new((0, 0)));
    // pub static ref IS_INIT_SIDE_BAR: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));
}
