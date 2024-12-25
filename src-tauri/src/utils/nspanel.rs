use crate::utils::monitor;
use tauri::{AppHandle, LogicalPosition, LogicalSize, Manager, WebviewWindow};
use tauri_nspanel::{panel_delegate, ManagerExt, WebviewWindowExt};

pub fn init(app: &tauri::App) {
    let handle = app.app_handle().to_owned();
    let window: WebviewWindow = handle.get_webview_window("main").unwrap();

    let panel = window.to_panel().unwrap();

    let delegate = panel_delegate!(MyPanelDelegate {
        window_did_become_key,
        window_did_resign_key
    });

    delegate.set_listener(Box::new(move |delegate_name: String| {
        match delegate_name.as_str() {
            "window_did_become_key" => {
                let app_name = handle.package_info().name.to_owned();
                println!("[info]: {:?} panel becomes key window!", app_name);
            }
            "window_did_resign_key" => {
                println!("[info]: panel resigned from key window!");
            }
            _ => (),
        }
    }));

    panel.set_delegate(delegate);
}

const WIN_HEIGHT: f64 = 322.0;

#[tauri::command]
pub fn toggle_panel(handle: AppHandle) {
    let panel = handle.get_webview_panel("main").unwrap();

    if panel.is_visible() {
        panel.order_out(None);
        return;
    }

    let monitor = monitor::get_active_monitor(&handle);
    if let Some(m) = monitor {
        let win = handle.get_webview_window("main").unwrap();
        let size = m.size().to_logical(m.scale_factor());
        let position = m.position();

        let _ = win.set_size(LogicalSize::new(size.width, WIN_HEIGHT));
        let _ = win.set_position(LogicalPosition::new(
            position.x as f64,
            size.height - WIN_HEIGHT + position.y as f64,
        ));
    }

    panel.show();
}

#[tauri::command]
pub fn close_panel(handle: AppHandle) {
    let panel = handle.get_webview_panel("main").unwrap();

    panel.released_when_closed(true);
    panel.close();
}
