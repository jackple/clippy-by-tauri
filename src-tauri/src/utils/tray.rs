use crate::utils::nspanel;
use tauri::path::BaseDirectory;
use tauri::Manager;
use tauri::{
    image,
    menu::{AboutMetadata, Menu, MenuItem, PredefinedMenuItem},
    tray::TrayIconBuilder,
};

pub fn init(app: &tauri::App) {
    let separator = PredefinedMenuItem::separator(app).unwrap();
    let open = MenuItem::with_id(app, "open", "打开面板", true, Some("Cmd+Shift+V")).unwrap();
    let about =
        PredefinedMenuItem::about(app, Some("关于clippy2"), Some(generate_metadata(&app))).unwrap();
    let quit = PredefinedMenuItem::quit(app, Some("退出clippy2")).unwrap();
    let menu = Menu::with_items(app, &[&open, &separator, &about, &quit]).unwrap();

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
            _ => {
                println!("menu item {:?} not handled", event.id);
            }
        })
        .tooltip("Cmd+Shift+V 打开")
        .build(app)
        .unwrap();
}

fn generate_metadata(app: &tauri::App) -> AboutMetadata {
    let icon = image::Image::from_path(
        app.path()
            .resolve("assets/icon.png", BaseDirectory::Resource)
            .unwrap(),
    )
    .unwrap();

    let pkg_info = app.package_info();
    let metadata = AboutMetadata {
        name: Some(pkg_info.name.clone()),
        authors: Some(vec![pkg_info.authors.to_string()]),
        version: Some(pkg_info.version.to_string()),
        short_version: None,
        comments: None,
        copyright: Some(String::from("Copyright © 2025 yidejia")),
        license: None,
        website: None,
        website_label: None,
        credits: None,
        icon: Some(icon),
    };

    metadata
}
