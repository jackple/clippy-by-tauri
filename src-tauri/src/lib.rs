use tauri::Manager;

mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|_app, _args, _cwd| {
            // 暂时无法解决dock icon再次出现的问题
            // https://github.com/tauri-apps/tauri/issues/12128
        }))
        .plugin(tauri_plugin_process::init())
        .plugin(
            tauri_plugin_updater::Builder::new()
                .target("macos-universal")
                .build(),
        )
        .plugin(tauri_nspanel::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            utils::nspanel::toggle_panel,
            utils::db::add_record,
            utils::db::get_records,
            utils::db::toggle_favorite,
            utils::clipboard_write::choose,
        ])
        .setup(|app: &mut tauri::App| {
            // 隐藏dock icon
            hide_dock_icon(app);

            utils::db::init(&app);
            utils::nspanel::init(&app);
            utils::global_shortcut::register(&app);
            utils::clipboard_read::init();
            utils::tray::init(&app);

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
