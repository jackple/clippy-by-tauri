use crate::utils::monitor;
use cocoa::appkit::{NSMainMenuWindowLevel, NSWindowCollectionBehavior};
use tauri::{AppHandle, LogicalPosition, LogicalSize, Manager, WebviewWindow};
use tauri_nspanel::{panel_delegate, ManagerExt, WebviewWindowExt};

pub fn init(app: &tauri::App) {
    let handle = app.app_handle().to_owned();
    let window: WebviewWindow = handle.get_webview_window("main").unwrap();

    let panel = window.to_panel().unwrap();
    let panel_ref = panel.clone();

    let delegate = panel_delegate!(MyPanelDelegate {
        window_did_become_key,
        window_did_resign_key,
    });

    delegate.set_listener(Box::new(move |delegate_name: String| {
        match delegate_name.as_str() {
            "window_did_become_key" => {
                println!("[info]: panel becomes key window!");
            }
            "window_did_resign_key" => {
                println!("[info]: panel resigned from key window!");
                panel_ref.order_out(None);
            }
            _ => (),
        }
    }));

    panel.set_delegate(delegate);

    // 允许全屏应用下使用
    // https://github.com/tauri-apps/tauri/issues/5566#issuecomment-2409184685
    // Set a higher level
    panel.set_level(NSMainMenuWindowLevel + 1);
    // Prevents your panel from activating the owing application
    panel.set_style_mask(1 << 7);
    // Allow your panel to join fullscreen spaces; you can tweak this configuration
    panel.set_collection_behaviour(
        NSWindowCollectionBehavior::NSWindowCollectionBehaviorCanJoinAllSpaces
            | NSWindowCollectionBehavior::NSWindowCollectionBehaviorStationary
            | NSWindowCollectionBehavior::NSWindowCollectionBehaviorFullScreenAuxiliary,
    );
}

const WIN_HEIGHT: f64 = 322.0;

#[tauri::command]
pub fn toggle_panel(handle: AppHandle) {
    let panel = handle.get_webview_panel("main").unwrap();

    if panel.is_visible() {
        panel.order_out(None);
        return;
    }

    let win = handle.get_webview_window("main").unwrap();
    let monitor = monitor::get_active_monitor(&handle);
    if let Some(m) = monitor {
        let size = m.size().to_logical(m.scale_factor());
        let position = m.position();

        win.set_size(LogicalSize::new(size.width, WIN_HEIGHT))
            .unwrap();
        win.set_position(LogicalPosition::new(
            position.x as f64,
            size.height - WIN_HEIGHT + position.y as f64,
        ))
        .unwrap();
    }

    panel.show();
    // 只有focus状态下才能触发window_did_resign_key
    win.set_focus().unwrap();
}

#[tauri::command]
pub fn close_panel(handle: AppHandle) {
    let panel = handle.get_webview_panel("main").unwrap();

    panel.released_when_closed(true);
    panel.close();
}
