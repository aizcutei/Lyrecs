use log::info;
use tauri::{SystemTray, CustomMenuItem, SystemTrayMenu, AppHandle, SystemTrayMenuItem, SystemTrayEvent, Manager};

pub fn tray_icon() -> SystemTray {

    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let setting = CustomMenuItem::new("setting".to_string(), "Setting");
    let search = CustomMenuItem::new("search".to_string(), "Search");
    let tray_menu = SystemTrayMenu::new()
        .add_item(setting)
        .add_item(search)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    SystemTray::new().with_menu(tray_menu)
}

pub fn tray_handler(app: &AppHandle, event: SystemTrayEvent) {

    match event {
        SystemTrayEvent::LeftClick {
            position: _,
            size: _,
            ..
        } => {
            info!("system tray received a left click");
        }
        SystemTrayEvent::RightClick {
            position: _,
            size: _,
            ..
        } => {
            info!("system tray received a right click");
        }
        SystemTrayEvent::DoubleClick {
            position: _,
            size: _,
            ..
        } => {
            info!("system tray received a double click");
        }
        SystemTrayEvent::MenuItemClick { id, .. } => {
            match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                "search" => {
                    let is_search_window_opened = app.get_window("search");
                    if is_search_window_opened.is_none() {
                        let search_window = tauri::WindowBuilder::new(
                            app,
                            "search",
                            tauri::WindowUrl::App("search".into())
                            ).title("Search").decorations(true).resizable(true).transparent(false).center().build().unwrap();
                    }
                }
                "setting" => {
                    let is_setting_window_opened = app.get_window("setting");
                    //Make sure only one setting window is opened
                    if is_setting_window_opened.is_none() {
                        let setting_window = tauri::WindowBuilder::new(
                            app,
                            "setting",
                            tauri::WindowUrl::App("setting".into())
                            ).title("Setting").decorations(false).resizable(true).transparent(true).center().build().unwrap();
                    }
                }
                _ => {}
            }
        }
        _ => {}
    }
}
