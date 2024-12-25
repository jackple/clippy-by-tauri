use tauri::Manager;

mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_sql::Builder::default().build())
        .plugin(tauri_nspanel::init())
        .invoke_handler(tauri::generate_handler![
            utils::nspanel::show_panel,
            utils::nspanel::hide_panel,
            utils::nspanel::close_panel,
        ])
        .setup(|app| {
            utils::nspanel::init(&app);
            utils::global_shortcut::register(&app);
            // debug(&app);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![utils::monitor::get_active_monitor,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[allow(dead_code)]
fn debug(app: &tauri::App) {
    let window = app.get_webview_window("main").unwrap();
    window.open_devtools();
}
