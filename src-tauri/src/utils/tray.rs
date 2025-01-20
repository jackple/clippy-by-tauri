use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
};

pub fn init(app: &tauri::App) {
    let quit_i = MenuItem::with_id(app, "quit", "退出clippy2", true, None::<&str>).unwrap();
    let menu = Menu::with_items(app, &[&quit_i]).unwrap();

    TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .show_menu_on_left_click(true)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "quit" => {
                println!("quit menu item was clicked");
                app.exit(0);
            }
            _ => {
                println!("menu item {:?} not handled", event.id);
            }
        })
        .build(app)
        .unwrap();
}
