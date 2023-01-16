#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{SystemTray, CustomMenuItem, SystemTrayMenu, SystemTrayEvent, Manager};

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "退出");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit);
    let system_tray = SystemTray::new()
        .with_menu(tray_menu);

    tauri::Builder::default()
        //单实例功能实现
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            let window = app.get_window("main").unwrap();
            //窗体最小化后显示窗体
            window.unminimize().unwrap();
            //使窗体获取焦点，显示在最顶部
            window.set_focus().unwrap();
        }))
        .system_tray(system_tray)
        .on_system_tray_event(|_, event| menu_handle(event))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn menu_handle(event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => {
            match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                _ => {}
            }
        }
        _ => {}
    }
}
