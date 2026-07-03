// System notification module
//
// Cross-platform desktop notifications for background task completion
// and permission requests when the main window is hidden.

use tauri::{AppHandle, Emitter, Manager, Runtime};

/// Send a desktop notification
pub fn notify<R: Runtime>(
    app: &AppHandle<R>,
    title: &str,
    body: &str,
) {
    // Emit to frontend – the frontend will use tauri-plugin-notification
    // to display the actual OS notification
    let _ = app.emit("background-notification", serde_json::json!({
        "title": title,
        "body": body,
    }));
}

/// Notify when a background agent task completes
pub fn notify_task_complete<R: Runtime>(
    app: &AppHandle<R>,
    session_id: &str,
    project_name: &str,
    runtime_name: &str,
) {
    // Only notify if main window is hidden
    if let Some(window) = app.get_webview_window("main") {
        let is_hidden = !window.is_visible().unwrap_or(true);
        if is_hidden {
            notify(
                app,
                &format!("{} 任务完成", runtime_name),
                &format!("项目 {} 的 Agent 已完成", project_name),
            );
        }
    }
}

/// Notify when a CLI requests user permission while window is hidden
pub fn notify_permission_request<R: Runtime>(
    app: &AppHandle<R>,
    session_id: &str,
    runtime_name: &str,
    action: &str,
) {
    if let Some(window) = app.get_webview_window("main") {
        let is_hidden = !window.is_visible().unwrap_or(true);
        if is_hidden {
            notify(
                app,
                &format!("{} 需要权限确认", runtime_name),
                &format!("请点击查看: {}", action),
            );
        }
    }
}
