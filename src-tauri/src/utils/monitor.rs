use core_graphics::display::CGDisplay;
use objc2_app_kit::NSEvent;
use tauri::AppHandle;

// 获取鼠标位置
// by https://github.com/enigo-rs/enigo/blob/main/examples/mouse.rs
fn get_mouse_location() -> (f64, f64) {
    let pt = unsafe { NSEvent::mouseLocation() };
    let (x, y_inv) = (pt.x as f64, pt.y as f64);
    (x, CGDisplay::main().pixels_high() as f64 - y_inv)
}

pub fn get_active_monitor(app_handle: &AppHandle) -> tauri::Monitor {
    let pos = get_mouse_location();
    let mon = app_handle.monitor_from_point(pos.0, pos.1);
    if let Ok(Some(mon)) = mon {
        return mon;
    }

    // 如果没找到，返回第一个屏幕的
    let monitors = app_handle.available_monitors().expect("获取显示器失败");
    monitors[0].clone()
}
