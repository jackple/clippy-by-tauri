use crate::utils::nspanel;
use tauri::Manager;
use tauri_plugin_global_shortcut::{Code, Modifiers, ShortcutState};

#[cfg(desktop)]
pub fn register(app: &tauri::App) {
    let _ = app.app_handle().plugin(
        tauri_plugin_global_shortcut::Builder::new()
            .with_shortcut("CommandOrControl+Shift+B")
            .unwrap()
            .with_handler(|app_handle, shortcut, event| {
                if event.state == ShortcutState::Pressed {
                    println!("shortcut: {:#?}", shortcut);
                    if shortcut.matches(Modifiers::SHIFT, Code::KeyV) {
                        nspanel::show_panel(app_handle.to_owned());
                    }
                }
            })
            .build(),
    );
}

// fn toggle_main_window(app: &tauri::App) {
//     let window = app.get_webview_window("main").unwrap();
//     // window.toggle();
// }
