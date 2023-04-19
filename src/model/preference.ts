export interface Preference {
    // 窗口模式 0为窗口模式,1为任务栏模式,2侧边栏模式
    window_mode: WindowMode,

    // 启用内置脚本 默认为false
    enable_internal_script: boolean,

    // 启用外置脚本 默认为false
    enable_extendsion_script: boolean,

    // 当点击窗口外侧自动隐藏窗口，默认自动隐藏
    auto_hide_when_click_outside: boolean,

    // 当前的访问页面的地址
    current_page_url: String,

    // 点击关闭退出应用
    exit_app: boolean,
}

export enum WindowMode {
    'Window',
    'TaskBar',
    // 侧边栏
    'SideBar',
    'QQ'
}
