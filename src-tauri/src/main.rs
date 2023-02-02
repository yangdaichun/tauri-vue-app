#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{SystemTray, CustomMenuItem, SystemTrayMenu, SystemTrayEvent, Manager, AppHandle, Runtime, Window, SystemTrayMenuItem, GlobalWindowEvent, WindowEvent};

const MAIN_LABEL:&str = "main";
fn main() {
    

    let quit = CustomMenuItem::new("quit".to_string(), "退出");
    let dev_tool = CustomMenuItem::new("devTool".to_string(), "调试工具");
    let tray_menu = SystemTrayMenu::new()
        .add_item(dev_tool)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);
    let system_tray = SystemTray::new()
        .with_menu(tray_menu);

    tauri::Builder::default()
        //单实例功能实现
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| single_instance(app)))
        // 采用CloseRequested 处理关闭提示消息，在自定义中调用close关闭窗体并没有触发CloseRequested事件，未确定原因
        // 暂时采用事件 方式实现
        .setup(| app | {
            let window: Window = app.get_window(MAIN_LABEL).unwrap();
            app.listen_global("app-exist",  move |_| app_exist(Some(&window)));
            Ok(())
        })
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| menu_handle(app, event))
        // 采用CloseRequested 处理关闭提示消息，在自定义中调用close关闭窗体并没有触发CloseRequested事件，未确定原因
        .on_window_event(|event| window_event(event))
        .run(tauri::generate_context!())
        .expect("error while running tauri application")
}

fn single_instance(app:&AppHandle) {
    let window = app.get_window(MAIN_LABEL).unwrap();
    //窗体最小化后显示窗体
    window.unminimize().unwrap();
    //使窗体获取焦点，显示在最顶部
    window.set_focus().unwrap();
}

fn menu_handle(app:&AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::DoubleClick {
            position: _,
            size: _,
            ..
          } => {
            let window = app.get_window(MAIN_LABEL).unwrap();
            //显示窗体
            window.unminimize().unwrap();
            window.show().unwrap(); 
            window.set_focus().unwrap();
          }
        SystemTrayEvent::MenuItemClick { id, .. } => {
            match id.as_str() {
                "quit" => {
                    let window = app.get_window(MAIN_LABEL).unwrap();
                    //关闭窗体
                    // window.close().unwrap();
                    app_exist(Some(&window))
                }
                "devTool" => {
                    let window = app.get_window(MAIN_LABEL).unwrap();
                    //打开调试工具
                    window.open_devtools();
                }
                _ => {}
            }
        }
        _ => {}
    }
}

fn window_event(event:GlobalWindowEvent){
    match event.event() {
        WindowEvent::CloseRequested { api, .. } => {
            let window = event.window().clone();
            if  window.label() == MAIN_LABEL {
                api.prevent_close();
                app_exist(Some(&window));
            }
        }
        _ => {}
    } 
}

fn app_exist<R: Runtime>(parent_window: Option<&Window<R>>) {
    tauri::api::dialog::confirm(parent_window, "退出", "确定要退出程序吗？",  | answer | {
        if answer {
            std::process::exit(-1);
        }
    })
}