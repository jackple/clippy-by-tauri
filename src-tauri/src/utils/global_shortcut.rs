use crate::utils::nspanel;
use tauri::Manager;
use tauri_plugin_global_shortcut::{Code, Modifiers, ShortcutState};

#[cfg(desktop)]
pub fn register(app: &tauri::App) {
    let _ = app.app_handle().plugin(
        tauri_plugin_global_shortcut::Builder::new()
            .with_shortcut("CommandOrControl+Shift+V")
            .unwrap()
            .with_handler(|app_handle, shortcut, event| {
                if event.state == ShortcutState::Pressed {
                    // println!("shortcut: {:#?}", shortcut);
                    if shortcut.matches(Modifiers::SHIFT | Modifiers::SUPER, Code::KeyV) {
                        nspanel::toggle_panel(app_handle.to_owned());
                    }
                }
            })
            .build(),
    );
}
