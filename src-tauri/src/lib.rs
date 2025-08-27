// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::fs;
use std::process::Command;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn convert_to_webp(image_path: String, output_path: String) -> Result<String, String> {
    // 使用cwebp命令行工具转换图片
    let output = Command::new("cwebp")
        .arg(&image_path)
        .arg("-o")
        .arg(&output_path)
        .output()
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_os::init()) // 初始化 tauri_plugin_os 插件
        .invoke_handler(tauri::generate_handler![
            greet,
            convert_to_webp,
            write_temp_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
