use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use cocoa::base::{id, nil};
use cocoa::foundation::NSString;
use lazy_static::lazy_static;
use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};
use std::fs;
use std::sync::Mutex;
use std::time::Duration;

use crate::utils::db::{add_record, RecordInput};
use crate::utils::optimize_img::optimize_img;

struct Record {
    record_type: Option<String>,
    value: Option<String>,
    img_bytes: Option<Vec<u8>>,
}

impl Record {
    // 更新记录，如果记录没有变化，则返回 false
    fn update(&mut self, record_type: &str, value: &str, img_bytes: Option<Vec<u8>>) -> bool {
        let record_type = Some(record_type.to_string());
        let value = Some(value.to_string());

        if self.record_type == record_type && self.value == value {
            return false;
        }
        self.record_type = record_type;
        self.value = value;
        self.img_bytes = img_bytes;
        true
    }

    fn is_same_img(&self, img_bytes: &[u8]) -> bool {
        if self.img_bytes.is_none() {
            return false;
        }
        self.img_bytes.as_ref().unwrap() == img_bytes
    }
}

lazy_static! {
    static ref LAST_RECORD: Mutex<Record> = Mutex::new(Record {
        record_type: None,
        value: None,
        img_bytes: None,
    });
}

pub fn init() {
    // 每秒检查一次剪贴板变化
    std::thread::spawn(move || {
        let runtime = tokio::runtime::Runtime::new().unwrap(); // 创建一个新的 Tokio 运行时
        runtime.block_on(async {
            loop {
                check().await; // 在异步上下文中调用
                std::thread::sleep(Duration::from_millis(300));
            }
        });
    });
}

pub async fn check() {
    let record_to_add = unsafe {
        objc::rc::autoreleasepool(|| {
            let pasteboard: id = msg_send![class!(NSPasteboard), generalPasteboard];
            let types: id = msg_send![pasteboard, types];

            // 检查是否包含文件路径
            let file_url_type = NSString::alloc(nil).init_str("public.file-url");
            let contains_file_urls: bool = msg_send![types, containsObject: file_url_type];
            if contains_file_urls {
                let item: *mut Object = msg_send![types, objectAtIndex: 0];
                let data: *mut Object = msg_send![pasteboard, dataForType: item];
                if data.is_null() {
                    return None;
                }
                let nsstring: *mut Object = msg_send![class!(NSString), alloc];
                // UTF8 encoding
                let nsstring: *mut Object = msg_send![nsstring, initWithData:data encoding:4];
                // 此时获取都的url是这样的格式: file:///.file/id=6571367.45435786
                let url_string = nsstring_to_rust_string(nsstring);
                // 把它转换为标准化的路径
                if let Some(path) = resolve_file_url(&url_string) {
                    if LAST_RECORD.lock().unwrap().update("file", &path, None) {
                        // 获取文件大小
                        if let Ok(metadata) = fs::metadata(path.clone()) {
                            let record = RecordInput {
                                record_type: "file".to_string(),
                                value: path,
                                thumbnail: None,
                                size: Some(metadata.len()),
                                img_size: None,
                            };
                            return Some(record);
                        }
                    }
                }
                return None;
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
                    let mut last_record = LAST_RECORD.lock().unwrap();
                    // 在encode前检测图片是不是同一张, 如果一样, encode没有意义
                    if last_record.is_same_img(&slice) {
                        return None;
                    }
                    let (thumbnail, img_size) = optimize_img(&slice).unwrap();
                    let img_base64 = STANDARD.encode(slice);
                    let _ = last_record.update("image", &img_base64, Some(slice.to_vec()));

                    let record = RecordInput {
                        record_type: "image".to_string(),
                        value: img_base64,
                        thumbnail: Some(thumbnail),
                        size: None,
                        img_size: Some(img_size),
                    };
                    return Some(record);
                }
                return None;
            }

            // 检查是否包含文字
            let text_type = NSString::alloc(nil).init_str("public.utf8-plain-text");
            let contains_text: bool = msg_send![types, containsObject: text_type];
            if contains_text {
                let string: id = msg_send![pasteboard, stringForType: text_type];
                if !string.is_null() {
                    let t = nsstring_to_rust_string(string);
                    if LAST_RECORD.lock().unwrap().update("text", &t, None) {
                        let record = RecordInput {
                            record_type: "text".to_string(),
                            value: t,
                            thumbnail: None,
                            size: None,
                            img_size: None,
                        };
                        return Some(record);
                    }
                }
            }
            None
        })
    };

    if let Some(record) = record_to_add {
        add_record(record).await.unwrap();
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
