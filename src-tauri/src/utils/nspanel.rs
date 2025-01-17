use crate::utils::monitor;
use cocoa::appkit::{NSMainMenuWindowLevel, NSWindowCollectionBehavior};
use lazy_static::lazy_static;
use std::sync::Mutex;
use tauri::{AppHandle, LogicalPosition, LogicalSize, Manager, WebviewWindow};
use tauri_nspanel::{panel_delegate, ManagerExt, WebviewWindowExt};

pub struct PanelState {
    visible: bool,
}

impl PanelState {
    fn show(&mut self) {
        self.visible = true;
    }

    fn hide(&mut self) {
        self.visible = false;
    }

    #[allow(dead_code)]
    pub fn is_visible(&self) -> bool {
        self.visible
    }
}

// 导出可供其他模块使用的PanelState, 使用Mutex保证线程安全
// 让其他模块可知道当前panel是否可见
lazy_static! {
    pub static ref PANEL_STATE: Mutex<PanelState> = Mutex::new(PanelState { visible: false });
}

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
                PANEL_STATE.lock().unwrap().show();
            }
            "window_did_resign_key" => {
                println!("[info]: panel resigned from key window!");
                panel_ref.order_out(None);
                PANEL_STATE.lock().unwrap().hide();
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
    let win: WebviewWindow = handle.get_webview_window("main").unwrap();

    if panel.is_visible() {
        panel.order_out(None);
        // 隐藏时将win的y坐标设置到tauri.conf.json中设置的y坐标, 窗口从一个屏幕转到另一个屏幕时, 窗口不会闪烁
        win.set_position(LogicalPosition::new(0.0, -100000.0))
            .unwrap();
        return;
    }

    let monitor = monitor::get_active_monitor(&handle);
    let size = monitor.size().to_logical(monitor.scale_factor());
    let position = monitor.position();

    win.set_size(LogicalSize::new(size.width, WIN_HEIGHT))
        .unwrap();
    win.set_position(LogicalPosition::new(
        position.x as f64,
        size.height - WIN_HEIGHT + position.y as f64,
    ))
    .unwrap();

    panel.show();
    // 只有focus状态下才能触发window_did_resign_key
    win.set_focus().unwrap();
}

// #[tauri::command]
// pub fn close_panel(handle: AppHandle) {
//     let panel = handle.get_webview_panel("main").unwrap();

//     panel.released_when_closed(true);
//     panel.close();
// }
