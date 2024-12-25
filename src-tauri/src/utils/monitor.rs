use cocoa::base::id;
use cocoa::foundation::{NSPoint, NSRect};
use objc::{class, msg_send, sel, sel_impl};
use tauri::AppHandle;

pub fn get_active_monitor(app_handle: &AppHandle) -> Option<tauri::Monitor> {
    unsafe {
        // 获取鼠标位置
        let mouse_location: NSPoint = msg_send![class!(NSEvent), mouseLocation];

        // 获取所有屏幕
        let screens: id = msg_send![class!(NSScreen), screens];
        let count: usize = msg_send![screens, count];

        let monitors = app_handle.available_monitors().expect("获取显示器失败");

        // 遍历所有屏幕找到包含鼠标的那个
        for i in 0..count {
            let screen: id = msg_send![screens, objectAtIndex: i];
            let frame: NSRect = msg_send![screen, frame];

            // 检查鼠标是否在这个屏幕的范围内
            if mouse_location.x >= frame.origin.x
                && mouse_location.x <= frame.origin.x + frame.size.width
                && mouse_location.y >= frame.origin.y
                && mouse_location.y <= frame.origin.y + frame.size.height
            {
                return get_monitor(frame, monitors);
            }
        }

        // 如果没找到，返回第一个屏幕的
        Some(monitors[0].clone())
    }
}

fn get_monitor(frame: NSRect, monitors: Vec<tauri::Monitor>) -> Option<tauri::Monitor> {
    for monitor in monitors {
        let size = monitor.size().to_logical::<f64>(monitor.scale_factor());
        let position = monitor.position();

        // 判断是否匹配
        if size.width == frame.size.width
            && size.height == frame.size.height
            && is_diff_less_than_10(position.x as f64, frame.origin.x)
            && is_diff_less_than_10(position.y as f64, frame.origin.y)
        {
            return Some(monitor.clone());
        }
    }

    None
}

fn is_diff_less_than_10(a: f64, b: f64) -> bool {
    (a - b).abs() < 10.0
}
