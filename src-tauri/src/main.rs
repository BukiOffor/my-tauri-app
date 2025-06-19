// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// use tauri::api::process::{Command, CommandEvent};
// use tauri::Window;

// #[tauri::command]
// fn start_server_sidecar(window: Window) {
//     let (mut rx, _) = Command::new_sidecar("server")
//         .expect("failed to create sidecar")
//         .spawn()
//         .expect("failed to spawn sidecar");

//     // Monitor server logs
//     tauri::async_runtime::spawn(async move {
//         while let Some(event) = rx.recv().await {
//             if let CommandEvent::Stdout(line) = event {
//                 println!("[server] {}", line); // Log to Tauri console
//                 let _ = window.emit("server-log", line); // Send to frontend
//             }
//         }
//     });
// }

use tauri::Manager;
use app::monitor_critical_sidecar;
fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();

            // Starting sidecar as a background task
            tauri::async_runtime::spawn(async move {
                // Start the sidecar server and monitor it
                monitor_critical_sidecar(window).await;
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri app");
}
