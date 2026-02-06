use tauri::menu::{MenuBuilder, MenuItemBuilder, CheckMenuItemBuilder};
use tauri::tray::{TrayIconBuilder, TrayIconEvent, MouseButton, MouseButtonState};
use std::path::PathBuf;
use tauri::image::Image as TauriImage;
use tauri::Manager;
use tauri::Emitter;
use std::sync::{Arc, Mutex};
use std::time::{Instant, Duration};

// Build the system tray and register event handlers.
//
// Behavior implemented:
// - Single left-click on the tray icon toggles the main window:
//     * If the main window is hidden -> show, unminimize, focus
//     * If the main window is visible -> hide it (hide to tray)
// - The tray menu contains a Toggle Autostart item and Quit.
pub fn build_system_tray(app: &tauri::AppHandle) -> tauri::Result<()> {
    // Menu items (constructed with the v2 MenuBuilder API)
    // Create a checkable menu item so its checked state represents the
    // autostart enabled state.
    let toggle = CheckMenuItemBuilder::with_id("toggle-autostart", "Toggle Autostart").checked(false).build(app)?;
    let quit = MenuItemBuilder::with_id("quit", "Quit").build(app)?;

    // A menu item that toggles the main window's visibility. We'll set its
    // initial text to "Show" or "Hide" based on the current window state.
    let show_hide = MenuItemBuilder::with_id("toggle-window", "Show").build(app)?;

    // If the main window exists, set initial menu text to match its state.
    if let Some(window) = app.get_webview_window("main") {
        if let Ok(visible) = window.is_visible() {
            let _ = show_hide.set_text(if visible { "Hide" } else { "Show" });
        }
    }

    let menu = MenuBuilder::new(app).items(&[&toggle, &show_hide, &quit]).build()?;

    // Set initial checked state from the autostart plugin if available and
    // also emit an event so the frontend can synchronize on startup.
    if let Ok(enabled) = crate::autostart::is_enabled(app) {
        let _ = toggle.set_checked(enabled);
        // Emit an event so the frontend can update its UI without polling.
        let _ = app.emit("autostart-changed", enabled);
    }

    // Use the packaged icon for the tray. Prefer the Windows .ico when
    // available; fall back to a PNG if not. The icon files live in
    // `src-tauri/icons` and are included in the Tauri bundle by default.
    let icon_path = PathBuf::from("icons/icon.ico");
    // Try to load an Image from the packaged icons. If loading fails, fall
    // back to building the tray without an explicit icon (some platforms
    // will still show the app icon from resources).
    let maybe_icon = if icon_path.exists() {
        TauriImage::from_path(icon_path.clone()).ok()
    } else {
        TauriImage::from_path(PathBuf::from("icons/icon.png")).ok()
    };

    // Debounce state for click handling
    let last_click = Arc::new(Mutex::new(Instant::now() - Duration::from_secs(1)));
    let debounce_ms = Duration::from_millis(200);

    let tray_builder = if let Some(icon) = maybe_icon {
        TrayIconBuilder::new().icon(icon).show_menu_on_left_click(false)
    } else {
        TrayIconBuilder::new().show_menu_on_left_click(false)
    };

    tray_builder
        .menu(&menu)
        // Handle icon interactions (clicks). We use the Click event and
        // implement toggle semantics: show/focus when hidden, hide when visible.
        .on_tray_icon_event({
            let show_hide = show_hide.clone();
            let last_click = last_click.clone();
            move |tray, event| match event {
                TrayIconEvent::Click { button, button_state, .. } => {
                    // Only handle mouse-up to match standard expectations.
                    if button_state != MouseButtonState::Up {
                        return;
                    }

                    // Debounce rapid clicks
                    {
                        let mut last = last_click.lock().unwrap();
                        if last.elapsed() < debounce_ms {
                            return;
                        }
                        *last = Instant::now();
                    }

                    if button == MouseButton::Left {
                        let app = tray.app_handle();
                        // Try to get the main window by the id "main" as declared in
                        // `tauri.conf.json`. If present, toggle its visibility/hide state.
                        if let Some(window) = app.get_webview_window("main") {
                            // is_visible returns a Result<bool>
                            if let Ok(visible) = window.is_visible() {
                                if visible {
                                    // If visible, hide the window (remove from taskbar).
                                    let _ = window.hide();
                                    let _ = show_hide.set_text("Show");
                                    let _ = tray.set_tooltip(Some(String::from("timeman — hidden")));
                                } else {
                                    // If not visible, show/unminimize and focus.
                                    let _ = window.show();
                                    let _ = window.unminimize();
                                    let _ = window.set_focus();
                                    let _ = show_hide.set_text("Hide");
                                    let _ = tray.set_tooltip(Some(String::from("timeman — visible")));
                                }
                            } else {
                                // If we cannot query visibility, fall back to showing the
                                // window to be safe.
                                let _ = window.show();
                                let _ = window.unminimize();
                                let _ = window.set_focus();
                                let _ = show_hide.set_text("Hide");
                                let _ = tray.set_tooltip(Some(String::from("timeman — visible")));
                            }
                        }
                    } else if button == MouseButton::Right {
                        // Let the system open the context menu on right click. Do
                        // not open it manually here to preserve native behavior.
                    }
                }
                _ => {}
            }
        })
        // Menu item handler
        .on_menu_event({
            // move a clone of the CheckMenuItem and the Show/Hide text item
            // into the closure so we can update them from Rust.
            let toggle = toggle.clone();
            let show_hide = show_hide.clone();
            move |app, event| {
                match event.id().as_ref() {
                    "toggle-autostart" => {
                        let app_handle = app.clone();
                        let toggle_clone = toggle.clone();
                        tauri::async_runtime::spawn(async move {
                            if let Ok(enabled) = crate::autostart::is_enabled(&app_handle) {
                                let _ = crate::autostart::set_enabled(&app_handle, !enabled);
                                let _ = toggle_clone.set_checked(!enabled);
                                // Notify frontend listeners that autostart changed.
                                let _ = app_handle.emit("autostart-changed", !enabled);
                            }
                        });
                    }
                    "toggle-window" => {
                        // Toggle the main window visibility from the menu item
                        let app_handle = app.clone();
                        let show_hide_clone = show_hide.clone();
                        tauri::async_runtime::spawn(async move {
                            if let Some(window) = app_handle.get_webview_window("main") {
                                if let Ok(visible) = window.is_visible() {
                                    if visible {
                                        let _ = window.minimize();
                                        let _ = show_hide_clone.set_text("Show");
                                    } else {
                                        let _ = window.unminimize();
                                        let _ = window.show();
                                        let _ = window.set_focus();
                                        let _ = show_hide_clone.set_text("Hide");
                                    }
                                }
                            }
                        });
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                }
            }
        })
        .build(app)?;

    // Set an initial tooltip so hovering the tray icon shows helpful text on
    // platforms that support it. Use an explicit generic to avoid type
    // inference issues across dependency crates.
    if let Some(window) = app.get_webview_window("main") {
        if let Ok(visible) = window.is_visible() {
            let _ = tray.set_tooltip::<String>(Some(String::from(if visible {
                "timeman — visible"
            } else {
                "timeman — hidden"
            })));
        } else {
            let _ = tray.set_tooltip::<String>(Some(String::from("timeman")));
        }
    } else {
        let _ = tray.set_tooltip::<String>(Some(String::from("timeman")));
    }

    Ok(())
}
