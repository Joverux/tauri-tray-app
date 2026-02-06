use tauri_plugin_autostart::ManagerExt;

pub fn enable_autostart(app: &tauri::App) -> tauri::Result<()> {
    let autostart_manager = app.autolaunch();
    autostart_manager.enable()?;
    Ok(())
}

pub fn is_enabled(app: &tauri::AppHandle) -> tauri::Result<bool> {
    let manager = app.autolaunch();
    manager.is_enabled()
}

pub fn set_enabled(app: &tauri::AppHandle, enabled: bool) -> tauri::Result<()> {
    let manager = app.autolaunch();
    if enabled {
        manager.enable()?;
    } else {
        manager.disable()?;
    }
    Ok(())
}
