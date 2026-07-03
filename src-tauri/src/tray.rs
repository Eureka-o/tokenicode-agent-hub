// System tray implementation for multi-platform support
//
// Provides minimize-to-tray functionality with configurable close behavior

use tauri::{
    image::Image,
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Emitter, Manager, Runtime, WebviewWindow,
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum CloseBehavior {
    Exit,
    MinimizeToTray,
    Ask,
}

impl Default for CloseBehavior {
    fn default() -> Self {
        CloseBehavior::MinimizeToTray
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TraySettings {
    pub close_behavior: CloseBehavior,
    pub start_hidden: bool,
    pub auto_start: bool,
    pub notify_on_completion: bool,
    pub notify_on_permission: bool,
}

impl Default for TraySettings {
    fn default() -> Self {
        Self {
            close_behavior: CloseBehavior::MinimizeToTray,
            start_hidden: false,
            auto_start: false,
            notify_on_completion: true,
            notify_on_permission: true,
        }
    }
}

/// Create and setup the system tray
pub fn create_tray<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<()> {
    // Create tray menu
    let show_item = MenuItem::with_id(app, "show", "显示主窗口", true, None::<&str>)?;
    let hide_item = MenuItem::with_id(app, "hide", "隐藏窗口", true, None::<&str>)?;
    let separator1 = PredefinedMenuItem::separator(app)?;
    let stop_all_item = MenuItem::with_id(app, "stop_all", "停止所有 Agent", true, None::<&str>)?;
    let separator2 = PredefinedMenuItem::separator(app)?;
    let usage_item = MenuItem::with_id(app, "usage", "打开用量面板", true, None::<&str>)?;
    let separator3 = PredefinedMenuItem::separator(app)?;
    let quit_item = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;

    let menu = Menu::with_items(
        app,
        &[
            &show_item,
            &hide_item,
            &separator1,
            &stop_all_item,
            &separator2,
            &usage_item,
            &separator3,
            &quit_item,
        ],
    )?;

    // Load tray icon
    let icon = load_tray_icon()?;

    // Build tray icon
    let _tray = TrayIconBuilder::new()
        .icon(icon)
        .menu(&menu)
        .tooltip("TOKENICODE Agent Hub")
        .on_menu_event(move |app, event| match event.id.as_ref() {
            "show" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            "hide" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.hide();
                }
            }
            "stop_all" => {
                // Emit event to frontend to stop all agents
                let _ = app.emit("tray-stop-all-agents", ());
            }
            "usage" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                    // Emit event to frontend to open usage panel
                    let _ = app.emit("tray-open-usage", ());
                }
            }
            "quit" => {
                // Request quit confirmation from frontend
                let _ = app.emit("tray-quit-request", ());
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    if window.is_visible().unwrap_or(false) {
                        let _ = window.hide();
                    } else {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
            }
        })
        .build(app)?;

    Ok(())
}

fn load_tray_icon() -> tauri::Result<Image> {
    // Try to load icon from embedded resources
    // Fallback to default icon if not found
    let icon_bytes = include_bytes!("../icons/32x32.png");
    Image::from_bytes(icon_bytes).map_err(|e| tauri::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))
}

/// Handle window close event based on settings
pub fn handle_close_request<R: Runtime>(
    window: &WebviewWindow<R>,
    settings: &TraySettings,
) -> bool {
    match settings.close_behavior {
        CloseBehavior::Exit => {
            // Let the window close normally (return false)
            false
        }
        CloseBehavior::MinimizeToTray => {
            // Hide window instead of closing
            let _ = window.hide();
            true // Prevent default close
        }
        CloseBehavior::Ask => {
            // Emit event to frontend for confirmation dialog
            let _ = window.emit("close-behavior-ask", ());
            true // Prevent default close until user responds
        }
    }
}
