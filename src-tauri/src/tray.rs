use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem, AppHandle};

pub fn build_system_tray() -> SystemTray {
    let toggle_autostart = CustomMenuItem::new("toggle-autostart".to_string(), "Toggle Autostart");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");

    let tray_menu = SystemTrayMenu::new()
        .add_item(toggle_autostart)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    SystemTray::new().with_menu(tray_menu)
}

pub fn on_tray_event(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "toggle-autostart" => {
                // call Rust autostart handler to toggle
                let app = app.clone();
                tauri::async_runtime::spawn(async move {
                    if let Ok(enabled) = crate::autostart::is_enabled(&app) {
                        let _ = crate::autostart::set_enabled(&app, !enabled);
                    }
                });
            }
            "quit" => {
                std::process::exit(0);
            }
            _ => {}
        },
        _ => {}
    }
}
