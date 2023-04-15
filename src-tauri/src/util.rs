#![allow(dead_code)]

/// 重启
pub fn reboot() {
    if let Ok(exe_path) = std::env::current_exe() {
        let _ = std::process::Command::new(exe_path).spawn();
        std::process::exit(0);
    }
}
