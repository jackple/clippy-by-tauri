use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use cocoa::base::{id, nil};
use cocoa::foundation::NSString;
use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};
use std::time::Duration;
use tauri::Manager;

use crate::utils::nspanel::PANEL_STATE;

pub fn init(app: &tauri::App) {
    // 每秒检查一次剪贴板变化
    std::thread::spawn(move || loop {
        check();
        std::thread::sleep(Duration::from_millis(2500));
    });
}

pub fn check() {
    println!(
        "panel state: {:?}",
        PANEL_STATE.lock().unwrap().is_visible()
    );

    unsafe {
        let pasteboard: id = msg_send![class!(NSPasteboard), generalPasteboard];
        let types: id = msg_send![pasteboard, types];

        let mut file_path = None;
        let mut image_base64 = None;
        let mut text_content = None;

        // 检查是否包含文件路径
        let file_url_type = NSString::alloc(nil).init_str("public.file-url");
        let contains_file_urls: bool = msg_send![types, containsObject: file_url_type];
        if contains_file_urls {
            let item: *mut Object = msg_send![types, objectAtIndex: 0];
            let data: *mut Object = msg_send![pasteboard, dataForType: item];
            if data.is_null() {
                return;
            }
            let nsstring: *mut Object = msg_send![class!(NSString), alloc];
            // UTF8 encoding
            let nsstring: *mut Object = msg_send![nsstring, initWithData:data encoding:4];
            // 此时获取都的url是这样的格式: file:///.file/id=6571367.45435786
            let url_string = nsstring_to_rust_string(nsstring);
            // 把它转换为标准化的路径
            if let Some(path) = resolve_file_url(&url_string) {
                file_path = Some(path);
            }
        }

        // 检查是否包含图片
        let image_type = NSString::alloc(nil).init_str("public.tiff");
        let contains_images: bool = msg_send![types, containsObject: image_type];
        if contains_images {
            let data: id = msg_send![pasteboard, dataForType: image_type];
            if !data.is_null() {
                let length: usize = msg_send![data, length];
                let bytes: *const u8 = msg_send![data, bytes];
                let slice = std::slice::from_raw_parts(bytes, length);
                image_base64 = Some(STANDARD.encode(slice));
            }
        }

        // 检查是否包含文字
        let text_type = NSString::alloc(nil).init_str("public.utf8-plain-text");
        let contains_text: bool = msg_send![types, containsObject: text_type];
        if contains_text {
            let string: id = msg_send![pasteboard, stringForType: text_type];
            if !string.is_null() {
                text_content = Some(nsstring_to_rust_string(string));
            }
        }

        // println!("file_path: {:?}", file_path);
        // println!("image_base64: {:?}", image_base64.is_some());
        // println!("text_content: {:?}", text_content);
    }
}

/// 将 `NSString` 转换为 Rust 字符串
fn nsstring_to_rust_string(nsstring: id) -> String {
    unsafe {
        let c_str: *const libc::c_char = msg_send![nsstring, UTF8String];
        std::ffi::CStr::from_ptr(c_str)
            .to_string_lossy()
            .into_owned()
    }
}

fn resolve_file_url(file_url: &str) -> Option<String> {
    unsafe {
        // 创建 NSURL 对象
        let nsurl_class = class!(NSURL);
        let nsurl: *mut Object =
            msg_send![nsurl_class, URLWithString: NSString::alloc(nil).init_str(file_url)];

        if nsurl.is_null() {
            return None;
        }
        // 标准化路径
        let resolved_url: *mut Object = msg_send![nsurl, URLByResolvingSymlinksInPath];

        // 获取标准化的路径字符串
        let file_path: *mut Object = msg_send![resolved_url, path];
        let path_cstr: *const i8 = msg_send![file_path, UTF8String];
        if path_cstr.is_null() {
            return None;
        }

        let path_str = std::ffi::CStr::from_ptr(path_cstr)
            .to_string_lossy()
            .into_owned();
        Some(path_str)
    }
}
