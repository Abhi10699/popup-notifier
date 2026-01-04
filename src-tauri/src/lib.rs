use async_std::task::sleep;
use std::time::Duration;
use tauri::{Manager, Url};

pub mod notification_popup;

const WAIT_MINUTES: u64 = 45;
const WAIT_SECONDS: u64 = 60 * WAIT_MINUTES;

#[tauri::command]
fn hide_window(webview_window: tauri::WebviewWindow) {
    println!("Hiding window")
    // webview_window.hide().unwrap();
    // println!(
    //     "window label = {window_label}",
    //     window_label = webview_window.label()
    // );
    // tauri::async_runtime::spawn(async move {
    //     let app_handle = webview_window.app_handle().clone();
    //     start_reminder_notification_wait(app_handle).await;
    // });
}

async fn start_reminder_notification_wait(app: tauri::AppHandle) {
    sleep(Duration::from_secs(WAIT_SECONDS)).await;
    app.get_webview_window("notification-window")
        .unwrap()
        .show()
        .unwrap();
}

#[tauri::command]
fn show_notification_window(webview_window: tauri::WebviewWindow) {
    let app_handle = webview_window.app_handle();
    notification_popup::Notification::new(app_handle);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.get_webview_window("notification-window")
                .unwrap()
                .close()
                .unwrap();
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            hide_window,
            show_notification_window
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
