use tauri::{AppHandle, Manager, WebviewWindow};
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

#[tauri::command]
pub fn show_panel(handle: AppHandle) {
    let panel = handle.get_webview_panel("main").unwrap();
    panel.show();
}

#[tauri::command]
pub fn hide_panel(handle: AppHandle) {
    let panel = handle.get_webview_panel("main").unwrap();
    panel.order_out(None);
}

#[tauri::command]
pub fn close_panel(handle: AppHandle) {
    let panel = handle.get_webview_panel("main").unwrap();
    panel.released_when_closed(true);
    panel.close();
}
