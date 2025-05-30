use crate::utils::{db, nspanel};
use tauri::path::BaseDirectory;
use tauri::{
    image,
    menu::{AboutMetadata, Menu, MenuItem, PredefinedMenuItem},
    tray::TrayIconBuilder,
};
use tauri::{Emitter, Manager};
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons, MessageDialogKind};

pub fn init(app: &tauri::App) {
    let separator = PredefinedMenuItem::separator(app).unwrap();
    let open = MenuItem::with_id(app, "open", "打开/隐藏面板", true, Some("Cmd+Shift+V")).unwrap();
    let clear_history =
        MenuItem::with_id(app, "clear_history", "清理历史记录", true, None::<&str>).unwrap();
    let about =
        PredefinedMenuItem::about(app, Some("关于clippy2"), Some(generate_metadata(&app))).unwrap();
    let quit = PredefinedMenuItem::quit(app, Some("退出clippy2")).unwrap();
    let menu = Menu::with_items(app, &[&open, &clear_history, &separator, &about, &quit]).unwrap();

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
            "clear_history" => {
                pre_clear_history(&app);
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
        copyright: Some(String::from("Copyright © 2025 jackple")),
        license: None,
        website: None,
        website_label: None,
        credits: None,
        icon: Some(icon),
    };

    metadata
}

fn pre_clear_history(app: &tauri::AppHandle) {
    let app_handle = app.clone();

    app.dialog()
        .message("确定要清理历史记录吗？")
        .kind(MessageDialogKind::Warning)
        .buttons(MessageDialogButtons::YesNo)
        .show(move |result| {
            if result {
                tauri::async_runtime::block_on(async {
                    match db::clear_history().await {
                        Ok(_) => {
                            // 通知渲染进程刷新数据
                            app_handle
                                .emit_to("main", "history-cleared", None::<&str>)
                                .unwrap();

                            app_handle
                                .dialog()
                                .message("历史记录已清理完成")
                                .title("成功")
                                .show(|_| {});
                        }
                        Err(e) => {
                            app_handle
                                .dialog()
                                .message(format!("清理失败: {}", e))
                                .kind(MessageDialogKind::Error)
                                .title("错误")
                                .show(|_| {});
                        }
                    }
                });
            }
        });
}
