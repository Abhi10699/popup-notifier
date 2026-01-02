use async_std::task::sleep;
use std::time::Duration;
use tauri::{Manager, PhysicalPosition, Position};

// #[derive(Debug)]
// struct AppState {
//     is_window_visible: bool,
// }

// impl AppState {
//     fn default() -> AppState {
//         AppState {
//             is_window_visible: false,
//         }
//     }
// }

#[tauri::command]
fn hide_window(webview_window: tauri::WebviewWindow) {
    // let mut lock = state.lock().unwrap();
    // println!("called hide window state: {:?}", lock.is_window_visible);
    // lock.is_window_visible = false;
    // println!("updated hide window state: {:?}", lock.is_window_visible);
    webview_window.hide().unwrap();
    tauri::async_runtime::spawn(async move {
        let app_handle = webview_window.app_handle().clone();
        start_reminder_notification_wait(app_handle).await;
    });
}

async fn start_reminder_notification_wait(app: tauri::AppHandle) {
    loop {
        sleep(Duration::from_secs(10)).await;
        app.get_webview_window("main").unwrap().show().unwrap();
        break;
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // init state

            // let app_state = Mutex::new(AppState::default());
            // app.manage(app_state);

            let main_window = app.get_webview_window("main").unwrap();
            let current_monitor = main_window.primary_monitor().unwrap().unwrap();
            let monitor_size = current_monitor.size();
            println!("Monitor size: {:?}", monitor_size);

            let current_window_size = main_window.outer_size().unwrap();
            const MARGIN: u32 = 40;

            let window_position = Position::new(PhysicalPosition {
                x: monitor_size.width - current_window_size.width - MARGIN,
                y: monitor_size.height - current_window_size.height - 200, // TODO: find a better way to set this value
            });

            main_window.set_position(window_position)?;
            main_window.hide().unwrap();

            let app_handle = app.handle().clone();
            //
            tauri::async_runtime::spawn(async move {
                start_reminder_notification_wait(app_handle).await;
            });

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![hide_window])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
