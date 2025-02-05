use crate::utils::nspanel;
use tauri::path::BaseDirectory;
use tauri::Manager;
use tauri::{
    image,
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::TrayIconBuilder,
};
use tauri_plugin_dialog::DialogExt;

pub fn init(app: &tauri::App) {
    let separator = PredefinedMenuItem::separator(app).unwrap();
    let open = MenuItem::with_id(app, "open", "打开面板", true, Some("Cmd+Shift+V")).unwrap();
    let about = MenuItem::with_id(app, "about", "关于clippy2", true, None::<&str>).unwrap();
    let quit = MenuItem::with_id(app, "quit", "退出clippy2", true, None::<&str>).unwrap();
    let menu = Menu::with_items(app, &[&open, &about, &separator, &quit]).unwrap();

    let icon = image::Image::from_path(
        app.path()
            .resolve("assets/tray.png", BaseDirectory::Resource)
            .unwrap(),
    )
    .unwrap();

    TrayIconBuilder::new()
        .icon(icon)
        .menu(&menu)
        .show_menu_on_left_click(true)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "open" => {
                println!("open panel by tray");
                nspanel::toggle_panel(app.clone());
            }
            "about" => {
                let pkg_info = app.package_info();
                let message = format!("版本: {}\n作者: {}", pkg_info.version, pkg_info.authors);
                app.dialog()
                    .message(message)
                    .title("关于clippy2")
                    .blocking_show();
            }
            "quit" => {
                println!("quit menu item was clicked");
                app.exit(0);
            }
            _ => {
                println!("menu item {:?} not handled", event.id);
            }
        })
        .tooltip("Cmd+Shift+V 打开")
        .build(app)
        .unwrap();
}
