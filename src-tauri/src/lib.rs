use tauri::Manager;

mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_sql::Builder::default().build())
        .plugin(tauri_nspanel::init())
        .invoke_handler(tauri::generate_handler![
            utils::nspanel::toggle_panel,
            utils::nspanel::close_panel,
        ])
        .setup(|app: &mut tauri::App| {
            // 隐藏dock icon
            hide_dock_icon(app);

            utils::nspanel::init(&app);
            utils::global_shortcut::register(&app);
            utils::clipboard::init(&app);

            // debug(&app);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[allow(dead_code)]
fn debug(app: &tauri::App) {
    let window = app.get_webview_window("main").unwrap();
    window.open_devtools();
}

fn hide_dock_icon(app: &mut tauri::App) {
    #[cfg(target_os = "macos")]
    app.set_activation_policy(tauri::ActivationPolicy::Accessory);
}
