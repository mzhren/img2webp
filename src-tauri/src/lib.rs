// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::fs;
use std::path::Path;
use std::process::Command;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn convert_to_webp(image_path: String, output_path: String, quality: Option<i32>) -> Result<String, String> {
    // 使用cwebp命令行工具转换图片
    let mut cmd = Command::new("cwebp");
    
    // 添加输入文件路径
    cmd.arg(&image_path);
    
    // 如果提供了质量参数，则添加质量设置
    if let Some(q) = quality {
        cmd.arg("-q").arg(q.to_string());
    }
    
    // 添加输出文件路径
    cmd.arg("-o").arg(&output_path);
    
    let output = cmd.output()
        .map_err(|e| format!("Failed to execute cwebp: {}", e))?;

    if output.status.success() {
        Ok(output_path)
    } else {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        Err(format!("cwebp conversion failed: {}", error_msg))
    }
}

#[tauri::command]
fn write_temp_file(path: String, data: Vec<u8>) -> Result<(), String> {
    match fs::write(&path, data) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to write temp file: {}", e)),
    }
}

// 添加新函数，用于获取文件目录
#[tauri::command]
fn get_parent_directory(file_path: String) -> Result<String, String> {
    let path = Path::new(&file_path);
    match path.parent() {
        Some(parent) => match parent.to_str() {
            Some(parent_str) => Ok(parent_str.to_string()),
            None => Err("无法将父目录路径转换为字符串".into())
        },
        None => Err("无法获取文件的父目录".into())
    }
}

#[tauri::command]
fn read_file_binary(path: String) -> Result<Vec<u8>, String> {
    match fs::read(&path) {
        Ok(data) => Ok(data),
        Err(e) => Err(format!("Failed to read file: {}", e)),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_os::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            convert_to_webp,
            write_temp_file,
            get_parent_directory,
            read_file_binary  // 注册新函数
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
