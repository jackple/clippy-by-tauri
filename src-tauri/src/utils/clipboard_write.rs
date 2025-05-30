use crate::utils;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use cocoa::base::{id, nil};
use cocoa::foundation::NSString;
use objc::{class, msg_send, sel, sel_impl};
use serde::Deserialize;
use std::path::Path;
use std::process::Command;

fn write_text(text: String) -> Result<(), String> {
    unsafe {
        objc::rc::autoreleasepool(|| {
            let pasteboard: id = msg_send![class!(NSPasteboard), generalPasteboard];
            let _: () = msg_send![pasteboard, clearContents];

            let text_type = NSString::alloc(nil).init_str("public.utf8-plain-text");
            let nsstring = NSString::alloc(nil).init_str(&text);
            let success: bool = msg_send![pasteboard, setString:nsstring forType:text_type];

            if success {
                Ok(())
            } else {
                Err("Failed to write text to clipboard".to_string())
            }
        })
    }
}

pub fn write_image(base64_str: String) -> Result<(), String> {
    let img_data = STANDARD
        .decode(base64_str)
        .map_err(|e| format!("Failed to decode base64: {}", e))?;

    unsafe {
        objc::rc::autoreleasepool(|| {
            let pasteboard: id = msg_send![class!(NSPasteboard), generalPasteboard];
            let _: () = msg_send![pasteboard, clearContents];

            let bytes = img_data.as_ptr() as *const std::os::raw::c_void;
            let nsdata: id = msg_send![class!(NSData), dataWithBytes:bytes length:img_data.len()];

            let image_type = NSString::alloc(nil).init_str("public.tiff");
            let success: bool = msg_send![pasteboard, setData:nsdata forType:image_type];

            if success {
                Ok(())
            } else {
                Err("Failed to write image to clipboard".to_string())
            }
        })
    }
}

pub fn write_file(file_path: String) -> Result<(), String> {
    // 检查文件是否存在
    let path = Path::new(&file_path);
    if !path.exists() {
        return Err("File not found".to_string());
    }

    let absoulte_path = path
        .canonicalize()
        .map_err(|e| e.to_string())?
        .to_string_lossy()
        .to_string();

    let script = format!(
        "set the clipboard to (POSIX file \"{}\")",
        absoulte_path.replace("\"", "\\\"")
    );

    let output = Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(format!(
            "Failed to write file to clipboard: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    Ok(())
}

#[derive(Debug, Deserialize)]
pub struct RecordInput {
    pub id: i64,
    pub record_type: String,
    pub value: String,
}

#[tauri::command]
pub async fn choose(record: RecordInput) -> Result<(), String> {
    if record.record_type == "text" {
        write_text(record.value)
    } else if record.record_type == "image" {
        let value = utils::db::get_record_value(record.id).await?;
        write_image(value)
    } else if record.record_type == "file" {
        write_file(record.value)
    } else {
        Err(format!("Unsupported record type: {}", record.record_type))
    }
}
