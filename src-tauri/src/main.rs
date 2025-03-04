// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Listener, Manager, WebviewUrl};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_sql::Builder::default().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .setup(|app| {
            let main_window = app.get_webview_window("main").unwrap();

            // 获取线程安全的 AppHandle
            let app_handle = app.handle().clone();
            main_window.listen("open-about", move |_| {
                // 检查是否已经存在 ID 为 "about" 的窗口
                if let Some(about_window) = app_handle.get_webview_window("about") {
                    // 如果窗口已经存在，将其置于桌面最前端显示
                    about_window.set_focus().unwrap();
                } else {
                    // 创建新窗口
                    tauri::WebviewWindowBuilder::new(
                        &app_handle, // 使用线程安全的 app_handle,
                        "about",
                        WebviewUrl::App("about.html".into()),
                    )
                    .title("关于")
                    .inner_size(350.0, 200.0)
                    .resizable(false) // 禁用窗口大小调整
                    .minimizable(false) // 禁用最小化按钮
                    .maximizable(false) // 禁用最大化按钮
                    .center() // 将窗口居中显示
                    .build()
                    .unwrap();
                }
            });
            Ok(())
        })
        // .plugin(tauri_plugin_store::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}